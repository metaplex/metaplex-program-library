"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.creatorAccountDataStruct = exports.CreatorAccountData = void 0;
var beet = require("@metaplex-foundation/beet");
var beetSolana = require("@metaplex-foundation/beet-solana");
/**
 * Holds the data for the {@link CreatorAccount} and provides de/serialization
 * functionality for that data
 * ToDo: To replace with Creator from mpl-token-metadata, after it starts to use beet
 */
var CreatorAccountData = /** @class */ (function () {
    function CreatorAccountData(address, verified, share) {
        this.address = address;
        this.verified = verified;
        this.share = share;
    }
    Object.defineProperty(CreatorAccountData, "byteSize", {
        /**
         * Returns the byteSize of a {@link Buffer} holding the serialized data of
         * {@link CreatorAccountData}
         */
        get: function () {
            return exports.creatorAccountDataStruct.byteSize;
        },
        enumerable: false,
        configurable: true
    });
    /**
     * Creates a {@link CreatorAccountData} instance from the provided args.
     */
    CreatorAccountData.fromArgs = function (args) {
        return new CreatorAccountData(args.address, args.verified, args.share);
    };
    /**
     * Deserializes the {@link CreatorAccountData} from the data of the provided {@link web3.AccountInfo}.
     * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
     */
    CreatorAccountData.fromAccountInfo = function (accountInfo, offset) {
        if (offset === void 0) { offset = 0; }
        return CreatorAccountData.deserialize(accountInfo.data, offset);
    };
    /**
     * Deserializes the {@link CreatorAccountData} from the provided data Buffer.
     * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
     */
    CreatorAccountData.deserialize = function (buf, offset) {
        if (offset === void 0) { offset = 0; }
        return exports.creatorAccountDataStruct.deserialize(buf, offset);
    };
    /**
     * Fetches the minimum balance needed to exempt an account holding
     * {@link CreatorAccountData} data from rent
     *
     * @param connection used to retrieve the rent exemption information
     */
    CreatorAccountData.getMinimumBalanceForRentExemption = function (connection, commitment) {
        return __awaiter(this, void 0, void 0, function () {
            return __generator(this, function (_a) {
                return [2 /*return*/, connection.getMinimumBalanceForRentExemption(CreatorAccountData.byteSize, commitment)];
            });
        });
    };
    /**
     * Determines if the provided {@link Buffer} has the correct byte size to
     * hold {@link CreatorAccountData} data.
     */
    CreatorAccountData.hasCorrectByteSize = function (buf, offset) {
        if (offset === void 0) { offset = 0; }
        return buf.byteLength - offset === CreatorAccountData.byteSize;
    };
    /**
     * Serializes the {@link CreatorAccountData} into a Buffer.
     * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
     */
    CreatorAccountData.prototype.serialize = function () {
        return exports.creatorAccountDataStruct.serialize(this);
    };
    /**
     * Returns a readable version of {@link CreatorAccountData} properties
     * and can be used to convert to JSON and/or logging
     */
    CreatorAccountData.prototype.pretty = function () {
        return {
            address: this.address.toBase58(),
            verified: this.verified,
            share: this.share,
        };
    };
    return CreatorAccountData;
}());
exports.CreatorAccountData = CreatorAccountData;
exports.creatorAccountDataStruct = new beet.BeetStruct([
    ['address', beetSolana.publicKey],
    ['verified', beet.u8],
    ['share', beet.u8],
], CreatorAccountData.fromArgs, 'CreatorAccountData');
