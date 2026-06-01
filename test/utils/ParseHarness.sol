// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {BurnMessageV2Lib} from "../../src/libraries/BurnMessageV2Lib.sol";

/// @notice External wrapper so tests can pass a `bytes memory` blob to the calldata-based parser.
contract ParseHarness {
    function parse(bytes calldata message)
        external
        pure
        returns (uint32, bytes32, bytes32, uint256, uint256, bytes memory)
    {
        (uint32 a, bytes32 b, bytes32 c, uint256 d, uint256 e, bytes calldata f) = BurnMessageV2Lib.parse(message);
        return (a, b, c, d, e, f);
    }
}
