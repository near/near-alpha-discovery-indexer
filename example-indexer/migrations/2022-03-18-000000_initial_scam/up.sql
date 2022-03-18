CREATE TYPE public.execution_outcome_status AS ENUM (
    'FAILURE',
    'SUCCESS'
);

CREATE TABLE public.events (
    block_height numeric(20,0) NOT NULL,
    block_hash text NOT NULL,
    block_timestamp numeric(20,0) NOT NULL,
    block_epoch_id text NOT NULL,
    receipt_id text NOT NULL,
    log_index integer NOT NULL,
    predecessor_id text NOT NULL,
    account_id text NOT NULL,
    status public.execution_outcome_status NOT NULL,
    event text NOT NULL
);

ALTER TABLE ONLY public.events
    ADD CONSTRAINT execution_outcome_receipt_pk PRIMARY KEY (receipt_id, log_index);

CREATE INDEX event_block_height_idx ON public.events USING btree (block_height);
CREATE INDEX event_account_id_idx ON public.events USING hash (account_id);
CREATE INDEX event_predecessor_id_idx ON public.events USING hash (predecessor_id);
