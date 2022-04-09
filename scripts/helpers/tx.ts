/* eslint-disable @typescript-eslint/no-explicit-any */
import * as fs from "fs";
import * as promptly from "promptly";
import {
  isTxError,
  Msg,
  MsgInstantiateContract,
  MsgStoreCode,
  Wallet,
} from "@terra-money/terra.js";

const DEFAULT_GAS_SETTINGS = {
  gasPrices: "0.15uusd",
  gasAdjustment: 1.4,
};

type AxiosError = {
  isAxiosError: boolean;
  response: any;
};

export function isAxiosError(error: any): error is AxiosError {
  return error && typeof error === "object" && error.hasOwnProperty("isAxiosError");
}

/**
 * @notice Send a transaction. Return result if successful, throw error if failed
 *
 * Use uusd for gas payment and mainnet gas prices for default. We could customize it to make the
 * function more flexible, but I'm too lazy for that
 */
export async function sendTransaction(signer: Wallet, msgs: Msg[]) {
  try {
    const tx = await signer.createAndSignTx({ msgs, ...DEFAULT_GAS_SETTINGS });
    const result = await signer.lcd.tx.broadcast(tx);
    if (isTxError(result)) {
      throw new Error("tx failed! raw log: " + result.raw_log);
    }
    return result;
  } catch (error: any) {
    let errorMsg;
    if (isAxiosError(error)) {
      errorMsg = error.response?.data.error || error.response?.data.message || "";
    } else errorMsg = error.message;
    throw new Error(errorMsg);
  }
}

/**
 * @notice Same with `sendTransaction`, but requires confirmation for CLI before broadcasting
 */
export async function sendTxWithConfirm(signer: Wallet, msgs: Msg[]) {
  try {
    const tx = await signer.createAndSignTx({ msgs, ...DEFAULT_GAS_SETTINGS });
    console.log("\n" + JSON.stringify(tx).replace(/\\/g, "") + "\n");

    const proceed = await promptly.confirm("Confirm transaction before broadcasting [y/N]:");
    if (!proceed) {
      console.log("User aborted!");
      process.exit(1);
    }

    const result = await signer.lcd.tx.broadcast(tx);
    if (isTxError(result)) {
      throw new Error(`tx failed! raw log: ${result.raw_log}`);
    }
    return result;
  } catch (error: any) {
    let errorMsg;
    if (isAxiosError(error)) {
      errorMsg = error.response?.data.error || error.response?.data.message || "";
    } else errorMsg = error.message;
    throw new Error(errorMsg);
  }
}

/**
 * @notice Upload contract code to LocalTerra, return code ID
 */
export async function storeCode(signer: Wallet, filePath: string) {
  const code = fs.readFileSync(filePath).toString("base64");
  const result = await sendTransaction(signer, [new MsgStoreCode(signer.key.accAddress, code)]);
  return parseInt(result.logs[0].eventsByType.store_code.code_id[0]);
}

/**
 * @notice Same with `storeCode`, but requires confirmation for CLI before broadcasting
 */
export async function storeCodeWithConfirm(signer: Wallet, filePath: string) {
  const code = fs.readFileSync(filePath).toString("base64");
  const result = await sendTxWithConfirm(signer, [new MsgStoreCode(signer.key.accAddress, code)]);
  return parseInt(result.logs[0].eventsByType.store_code.code_id[0]);
}

/**
 * @notice Instantiate a contract from an existing code ID, return the result
 * @dev The contract is set to be upgradable with the deployer as admin
 */
export async function instantiateContract(signer: Wallet, codeId: number, initMsg: object) {
  const result = await sendTransaction(signer, [
    new MsgInstantiateContract(signer.key.accAddress, signer.key.accAddress, codeId, initMsg),
  ]);
  return result;
}

/**
 * @notice Same with `instantiateContract`, but requires confirmation for CLI before broadcasting
 */
export async function instantiateWithConfirm(signer: Wallet, codeId: number, initMsg: object) {
  const result = await sendTxWithConfirm(signer, [
    new MsgInstantiateContract(signer.key.accAddress, signer.key.accAddress, codeId, initMsg),
  ]);
  return result;
}
