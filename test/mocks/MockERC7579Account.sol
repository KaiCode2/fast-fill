// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice Minimal ERC-7579 smart account stand-in. `executeFromExecutor` decodes a single
///         `(target, value, callData)` execution and performs it — enough to demonstrate that the
///         IntentExecutor ran the user's signed ops on the account (e.g. moving the delivered funds).
contract MockERC7579Account {
    event Executed(address indexed target, uint256 value, bytes data);

    /// @dev `executionCalldata = abi.encode(address target, uint256 value, bytes callData)`.
    function executeFromExecutor(bytes32, bytes calldata executionCalldata) external returns (bytes[] memory ret) {
        (address target, uint256 value, bytes memory data) = abi.decode(executionCalldata, (address, uint256, bytes));
        (bool ok, bytes memory out) = target.call{value: value}(data);
        require(ok, "ACCOUNT_EXEC_FAIL");
        emit Executed(target, value, data);
        ret = new bytes[](1);
        ret[0] = out;
    }

    receive() external payable {}
}
