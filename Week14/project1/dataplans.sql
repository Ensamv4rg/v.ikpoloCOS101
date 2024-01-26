--
-- PostgreSQL database dump
--

-- Dumped from database version 16.1
-- Dumped by pg_dump version 16.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: dataplan_table; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan_table (
    data_id integer NOT NULL,
    data_size_in_gb real NOT NULL,
    data_duration_days integer NOT NULL,
    data_price_naira real NOT NULL
);


ALTER TABLE public.dataplan_table OWNER TO postgres;

--
-- Data for Name: dataplan_table; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan_table (data_id, data_size_in_gb, data_duration_days, data_price_naira) FROM stdin;
1	0.35	2	200
2	1.8	14	500
3	3.9	30	1000
4	7.5	30	1500
5	9.2	30	2000
6	10.8	30	2500
7	14	30	3000
8	18	30	4000
9	24	30	5000
10	29.9	30	8000
11	50	30	10000
\.


--
-- Name: dataplan_table dataplan_table_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan_table
    ADD CONSTRAINT dataplan_table_pkey PRIMARY KEY (data_id);


--
-- PostgreSQL database dump complete
--

