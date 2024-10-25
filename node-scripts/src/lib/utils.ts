import "dotenv/config";
import { env } from "process";
import {
  Account,
  Aptos,
  AptosConfig,
  Ed25519PrivateKey,
  Network,
} from "@aptos-labs/ts-sdk";
import { createSurfClient } from "@thalalabs/surf";
import { Client } from "pg";

import { ABI } from "./abi/message_board_abi";
import { neon } from "@neondatabase/serverless";

const APTOS_CLIENT = new Aptos(
  new AptosConfig({
    network: Network.TESTNET,
  })
);

const SURF_CLIENT = createSurfClient(APTOS_CLIENT).useABI(ABI);

const POSTGRES_CLIENT = neon(env.DATABASE_URL!);

export const getAptosClient = () => APTOS_CLIENT;

export const getSurfClient = () => SURF_CLIENT;

export const getAccount = () => {
  if (!env.PRIVATE_KEY && env.PRIVATE_KEY === "to_fill") {
    throw new Error("Please fill in your private key");
  }

  return Account.fromPrivateKey({
    privateKey: new Ed25519PrivateKey(env.PRIVATE_KEY!),
  });
};

export const getPostgresClient = () => POSTGRES_CLIENT;