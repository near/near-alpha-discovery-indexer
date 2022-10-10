CREATE TABLE public.receipts (
    block_height numeric(20,0) NOT NULL,
    block_hash text NOT NULL,
    block_timestamp numeric(20,0) NOT NULL,
    block_epoch_id text NOT NULL,
    outcome_index integer NOT NULL,
    receipt_id text NOT NULL,
    index_in_receipt integer NOT NULL,
    signer_public_key text NOT NULL,
    signer_id text NOT NULL,
    predecessor_id text NOT NULL,
    account_id text NOT NULL,
    status public.execution_outcome_status NOT NULL,
    deposit numeric(40,0) NOT NULL,
    gas numeric(20,0) NOT NULL,
    method_name text NOT NULL,
    args text NOT NULL
);

ALTER TABLE ONLY public.receipts
    ADD CONSTRAINT receipts_execution_outcome_receipt_pk PRIMARY KEY (receipt_id, index_in_receipt);

CREATE INDEX receipts_block_height_idx ON public.receipts USING btree (block_height, outcome_index);
CREATE INDEX receipts_account_id_idx ON public.receipts USING hash (account_id);
CREATE INDEX receipts_predecessor_id_idx ON public.receipts USING hash (predecessor_id);
