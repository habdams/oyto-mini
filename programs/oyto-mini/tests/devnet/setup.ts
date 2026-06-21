import * as anchor from "@anchor-lang/core";

export const provider = anchor.AnchorProvider.env();

anchor.setProvider(provider);

export const connection = provider.connection;

export const wallet = provider.wallet;
