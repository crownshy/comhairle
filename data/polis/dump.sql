--
-- PostgreSQL database dump
--

-- Dumped from database version 13.4
-- Dumped by pg_dump version 13.4

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

--
-- Name: get_times_for_most_recent_visible_comments(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.get_times_for_most_recent_visible_comments() RETURNS TABLE(zid integer, modified bigint)
    LANGUAGE sql
    AS $$
    select zid, max(modified) from (select comments.*, conversations.strict_moderation from comments left join conversations on comments.zid = conversations.zid) as c where c.mod >= (CASE WHEN c.strict_moderation=TRUE then 1 else 0 END) group by c.zid order by c.zid;
$$;


ALTER FUNCTION public.get_times_for_most_recent_visible_comments() OWNER TO postgres;

--
-- Name: get_visible_comments(integer); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.get_visible_comments(the_zid integer) RETURNS TABLE(tid integer, mod integer, strict_moderation boolean)
    LANGUAGE sql
    AS $$
    select comments.tid, comments.mod, conversations.strict_moderation from comments left join conversations on comments.zid = conversations.zid where active = true and mod >= (CASE WHEN strict_moderation=TRUE then 1 else 0 END) and comments.zid = the_zid;
$$;


ALTER FUNCTION public.get_visible_comments(the_zid integer) OWNER TO postgres;

--
-- Name: now_as_millis(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.now_as_millis() RETURNS bigint
    LANGUAGE plpgsql
    AS $$
        DECLARE
            temp TIMESTAMP := now();
        BEGIN
            -- NOTE: milliseconds includes the seconds, so subtracting seconds from milliseconds
            -- SEE: http://www.postgresql.org/docs/8.4/static/functions-datetime.html
            RETURN 1000*FLOOR(EXTRACT(EPOCH FROM temp)) + FLOOR(EXTRACT(MILLISECONDS FROM temp)) - 1000*FLOOR(EXTRACT(SECOND FROM temp));
        END;
$$;


ALTER FUNCTION public.now_as_millis() OWNER TO postgres;

--
-- Name: pid_auto(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.pid_auto() RETURNS trigger
    LANGUAGE plpgsql STRICT
    AS $$
DECLARE
    _magic_id constant int := 873791983; -- This is a magic key used for locking conversation row-sets within the participants table. TODO keep track of these
    _conversation_id int;
BEGIN
    _conversation_id = NEW.zid;

    -- Obtain an advisory lock on the participants table, limited to this conversation
    PERFORM pg_advisory_lock(_magic_id, _conversation_id);

    SELECT  COALESCE(MAX(pid) + 1, 0) -- Start with comment id of 0
    INTO    NEW.pid
    FROM    participants
    WHERE   zid = NEW.zid;

    -- Duplicate participant_count to the conversations table to speed up conversationsView queries.
    UPDATE conversations
    SET participant_count = NEW.pid + 1
    WHERE zid = NEW.zid;

    RETURN NEW;
END;
$$;


ALTER FUNCTION public.pid_auto() OWNER TO postgres;

--
-- Name: pid_auto_unlock(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.pid_auto_unlock() RETURNS trigger
    LANGUAGE plpgsql STRICT
    AS $$
DECLARE
    _magic_id constant int := 873791983;
    _conversation_id int;
BEGIN
    _conversation_id = NEW.zid;

    -- Release the lock.
    PERFORM pg_advisory_unlock(_magic_id, _conversation_id);

    RETURN NEW;
END;
$$;


ALTER FUNCTION public.pid_auto_unlock() OWNER TO postgres;

--
-- Name: random_polis_site_id(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.random_polis_site_id() RETURNS text
    LANGUAGE sql
    AS $$
-- 18 so it's 32 long, not much thought went into this so far
SELECT 'polis_site_id_' || random_string(18);
$$;


ALTER FUNCTION public.random_polis_site_id() OWNER TO postgres;

--
-- Name: random_string(integer); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.random_string(integer) RETURNS text
    LANGUAGE sql
    AS $_$
SELECT array_to_string(
    ARRAY (
        SELECT substring(
            '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz'
            FROM (ceil(random()*62))::int FOR 1
        )
        FROM generate_series(1, $1)
    ),
    ''
)
$_$;


ALTER FUNCTION public.random_string(integer) OWNER TO postgres;

--
-- Name: tid_auto(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.tid_auto() RETURNS trigger
    LANGUAGE plpgsql STRICT
    AS $$
DECLARE
    _magic_id constant int := 873791984; -- This is a magic key used for locking conversation row-sets within the comments table. TODO keep track of these
    _conversation_id int;
BEGIN
    _conversation_id = NEW.zid;

    -- Obtain an advisory lock on the comments table, limited to this conversation
    PERFORM pg_advisory_lock(_magic_id, _conversation_id);

    SELECT  COALESCE(MAX(tid) + 1, 0) -- Start with comment id of 0
    INTO    NEW.tid
    FROM    comments
    WHERE   zid = NEW.zid;

    RETURN NEW;
END;
$$;


ALTER FUNCTION public.tid_auto() OWNER TO postgres;

--
-- Name: tid_auto_unlock(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.tid_auto_unlock() RETURNS trigger
    LANGUAGE plpgsql STRICT
    AS $$
DECLARE
    _magic_id constant int := 873791984;
    _conversation_id int;
BEGIN
    _conversation_id = NEW.zid;

    -- Release the lock.
    PERFORM pg_advisory_unlock(_magic_id, _conversation_id);

    RETURN NEW;
END;
$$;


ALTER FUNCTION public.tid_auto_unlock() OWNER TO postgres;

--
-- Name: to_millis(timestamp with time zone); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.to_millis(t timestamp with time zone) RETURNS bigint
    LANGUAGE plpgsql
    AS $$
        BEGIN
            RETURN 1000*FLOOR(EXTRACT(EPOCH FROM t)) + FLOOR(EXTRACT(MILLISECONDS FROM t)) - 1000*FLOOR(EXTRACT(SECOND FROM t));
        END;
$$;


ALTER FUNCTION public.to_millis(t timestamp with time zone) OWNER TO postgres;

--
-- Name: to_zid(text); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.to_zid(associated_zinvite text) RETURNS integer
    LANGUAGE plpgsql
    AS $$
        BEGIN
            RETURN (select zid from zinvites where zinvite = associated_zinvite);
        END;
$$;


ALTER FUNCTION public.to_zid(associated_zinvite text) OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: apikeysndvweifu; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.apikeysndvweifu (
    uid integer NOT NULL,
    apikey character varying(32) NOT NULL,
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.apikeysndvweifu OWNER TO postgres;

--
-- Name: auth_tokens; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.auth_tokens (
    token character varying(32),
    uid integer,
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.auth_tokens OWNER TO postgres;

--
-- Name: beta; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.beta (
    name character varying(999),
    email character varying(200),
    organization character varying(200),
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.beta OWNER TO postgres;

--
-- Name: comment_translations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.comment_translations (
    zid integer NOT NULL,
    tid integer NOT NULL,
    src integer NOT NULL,
    txt character varying(9999) NOT NULL,
    lang character varying(10) NOT NULL,
    created bigint DEFAULT public.now_as_millis(),
    modified bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.comment_translations OWNER TO postgres;

--
-- Name: comments; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.comments (
    tid integer NOT NULL,
    zid integer NOT NULL,
    pid integer NOT NULL,
    uid integer NOT NULL,
    created bigint DEFAULT public.now_as_millis(),
    modified bigint DEFAULT public.now_as_millis(),
    txt character varying(1000) NOT NULL,
    velocity real DEFAULT 1 NOT NULL,
    mod integer DEFAULT 0 NOT NULL,
    lang character varying(10),
    lang_confidence real,
    active boolean DEFAULT true NOT NULL,
    is_meta boolean DEFAULT false NOT NULL,
    tweet_id bigint,
    quote_src_url character varying(1000),
    anon boolean DEFAULT false NOT NULL,
    is_seed boolean DEFAULT false NOT NULL
);


ALTER TABLE public.comments OWNER TO postgres;

--
-- Name: contexts; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.contexts (
    context_id integer NOT NULL,
    name character varying(300),
    creator integer,
    is_public boolean DEFAULT false,
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.contexts OWNER TO postgres;

--
-- Name: contexts_context_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.contexts_context_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.contexts_context_id_seq OWNER TO postgres;

--
-- Name: contexts_context_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.contexts_context_id_seq OWNED BY public.contexts.context_id;


--
-- Name: contributer_agreement_signatures; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.contributer_agreement_signatures (
    uid integer,
    name character varying(746) NOT NULL,
    company_name character varying(746),
    github_id character varying(256),
    email character varying(256) NOT NULL,
    agreement_version integer NOT NULL,
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.contributer_agreement_signatures OWNER TO postgres;

--
-- Name: conversation_translations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.conversation_translations (
    zid integer NOT NULL,
    src integer NOT NULL,
    topic character varying(9999) NOT NULL,
    description character varying(9999) NOT NULL,
    lang character varying(10) NOT NULL,
    created bigint DEFAULT public.now_as_millis(),
    modified bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.conversation_translations OWNER TO postgres;

--
-- Name: conversations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.conversations (
    zid integer NOT NULL,
    topic character varying(1000),
    description character varying(50000),
    link_url character varying(9999),
    parent_url character varying(9999),
    upvotes integer DEFAULT 1 NOT NULL,
    participant_count integer DEFAULT 0,
    is_anon boolean DEFAULT true,
    is_active boolean DEFAULT false,
    is_draft boolean DEFAULT false,
    is_public boolean DEFAULT true,
    is_data_open boolean DEFAULT false,
    profanity_filter boolean DEFAULT true,
    spam_filter boolean DEFAULT true,
    strict_moderation boolean DEFAULT false,
    prioritize_seed boolean DEFAULT false,
    vis_type integer DEFAULT 0 NOT NULL,
    write_type integer DEFAULT 1 NOT NULL,
    help_type integer DEFAULT 1 NOT NULL,
    write_hint_type integer DEFAULT 1 NOT NULL,
    style_btn character varying(500),
    socialbtn_type integer DEFAULT 0 NOT NULL,
    subscribe_type integer DEFAULT 1 NOT NULL,
    branding_type integer DEFAULT 1 NOT NULL,
    bgcolor character varying(20),
    help_bgcolor character varying(20),
    help_color character varying(20),
    email_domain character varying(200),
    use_xid_whitelist boolean DEFAULT false,
    owner integer,
    org_id integer,
    context character varying(1000),
    course_id integer,
    owner_sees_participation_stats boolean DEFAULT false,
    auth_needed_to_vote boolean,
    auth_needed_to_write boolean,
    auth_opt_fb boolean,
    auth_opt_tw boolean,
    auth_opt_allow_3rdparty boolean,
    modified bigint DEFAULT public.now_as_millis(),
    created bigint DEFAULT public.now_as_millis(),
    importance_enabled boolean DEFAULT false NOT NULL
);


ALTER TABLE public.conversations OWNER TO postgres;

--
-- Name: conversations_zid_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.conversations_zid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.conversations_zid_seq OWNER TO postgres;

--
-- Name: conversations_zid_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.conversations_zid_seq OWNED BY public.conversations.zid;


--
-- Name: courses; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.courses (
    course_id integer NOT NULL,
    topic character varying(1000),
    description character varying(1000),
    owner integer,
    course_invite character varying(32),
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.courses OWNER TO postgres;

--
-- Name: courses_course_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.courses_course_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.courses_course_id_seq OWNER TO postgres;

--
-- Name: courses_course_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.courses_course_id_seq OWNED BY public.courses.course_id;


--
-- Name: crowd_mod; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.crowd_mod (
    zid integer NOT NULL,
    pid integer NOT NULL,
    tid integer NOT NULL,
    created bigint DEFAULT public.now_as_millis(),
    as_important boolean,
    as_factual boolean,
    as_feeling boolean,
    as_notmyfeeling boolean,
    as_notgoodidea boolean,
    as_notfact boolean,
    as_unsure boolean,
    as_spam boolean,
    as_abusive boolean,
    as_offtopic boolean
);


ALTER TABLE public.crowd_mod OWNER TO postgres;

--
-- Name: demographic_data; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.demographic_data (
    uid integer,
    fb_gender integer,
    ms_birth_year_estimate_fb integer,
    ms_gender_estimate_fb integer,
    fb_timestamp bigint DEFAULT public.now_as_millis(),
    ms_fb_timestamp bigint DEFAULT public.now_as_millis(),
    ms_response character varying(9999),
    gender_guess integer,
    birth_year_guess integer
);


ALTER TABLE public.demographic_data OWNER TO postgres;

--
-- Name: einvites; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.einvites (
    einvite character varying(100) NOT NULL,
    email character varying(999),
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.einvites OWNER TO postgres;

--
-- Name: email_validations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.email_validations (
    email character varying(999),
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.email_validations OWNER TO postgres;

--
-- Name: event_ptpt_no_more_comments; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.event_ptpt_no_more_comments (
    zid integer NOT NULL,
    pid integer NOT NULL,
    votes_placed smallint NOT NULL,
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.event_ptpt_no_more_comments OWNER TO postgres;

--
-- Name: facebook_friends; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.facebook_friends (
    uid integer NOT NULL,
    friend integer NOT NULL
);


ALTER TABLE public.facebook_friends OWNER TO postgres;

--
-- Name: facebook_users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.facebook_users (
    uid integer NOT NULL,
    fb_user_id text,
    fb_name character varying(9999),
    fb_link character varying(9999),
    fb_public_profile text,
    fb_login_status text,
    fb_auth_response text,
    fb_access_token text,
    fb_granted_scopes text,
    fb_location_id character varying(100),
    location character varying(9999),
    response text,
    fb_friends_response text,
    created bigint DEFAULT public.now_as_millis(),
    modified bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.facebook_users OWNER TO postgres;

--
-- Name: inviters; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.inviters (
    inviter_uid integer,
    invited_email character varying(999),
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.inviters OWNER TO postgres;

--
-- Name: jianiuevyew; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.jianiuevyew (
    uid integer NOT NULL,
    pwhash character varying(128) NOT NULL
);


ALTER TABLE public.jianiuevyew OWNER TO postgres;

--
-- Name: math_bidtopid; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.math_bidtopid (
    zid integer NOT NULL,
    math_env character varying(999) NOT NULL,
    math_tick bigint DEFAULT '-1'::integer NOT NULL,
    data jsonb NOT NULL,
    modified bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.math_bidtopid OWNER TO postgres;

--
-- Name: math_cache; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.math_cache (
    zid integer NOT NULL,
    math_env character varying(999) NOT NULL,
    data jsonb NOT NULL,
    modified bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.math_cache OWNER TO postgres;

--
-- Name: math_exportstatus; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.math_exportstatus (
    zid integer NOT NULL,
    math_env character varying(999) NOT NULL,
    filename character varying(9999) NOT NULL,
    data jsonb NOT NULL,
    modified bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.math_exportstatus OWNER TO postgres;

--
-- Name: math_main; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.math_main (
    zid integer NOT NULL,
    math_env character varying(999) NOT NULL,
    data jsonb NOT NULL,
    last_vote_timestamp bigint NOT NULL,
    caching_tick bigint DEFAULT 0 NOT NULL,
    math_tick bigint DEFAULT '-1'::integer NOT NULL,
    modified bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.math_main OWNER TO postgres;

--
-- Name: math_profile; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.math_profile (
    zid integer NOT NULL,
    math_env character varying(999) NOT NULL,
    data jsonb NOT NULL,
    modified bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.math_profile OWNER TO postgres;

--
-- Name: math_ptptstats; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.math_ptptstats (
    zid integer NOT NULL,
    math_env character varying(999) NOT NULL,
    math_tick bigint DEFAULT '-1'::integer NOT NULL,
    data jsonb NOT NULL,
    modified bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.math_ptptstats OWNER TO postgres;

--
-- Name: math_report_correlationmatrix; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.math_report_correlationmatrix (
    rid bigint NOT NULL,
    math_env character varying(999) NOT NULL,
    data jsonb,
    math_tick bigint DEFAULT '-1'::integer NOT NULL,
    modified bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.math_report_correlationmatrix OWNER TO postgres;

--
-- Name: math_ticks; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.math_ticks (
    zid integer,
    math_tick bigint DEFAULT 0 NOT NULL,
    caching_tick bigint DEFAULT 0 NOT NULL,
    math_env character varying(999) NOT NULL,
    modified bigint DEFAULT public.now_as_millis() NOT NULL
);


ALTER TABLE public.math_ticks OWNER TO postgres;

--
-- Name: metrics; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.metrics (
    uid integer,
    type integer NOT NULL,
    dur integer,
    hashedpc integer,
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.metrics OWNER TO postgres;

--
-- Name: notification_tasks; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.notification_tasks (
    zid integer NOT NULL,
    modified bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.notification_tasks OWNER TO postgres;

--
-- Name: oinvites; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.oinvites (
    oinvite character varying(300) NOT NULL,
    note character varying(999),
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.oinvites OWNER TO postgres;

--
-- Name: page_ids; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.page_ids (
    site_id character varying(100) NOT NULL,
    page_id character varying(100) NOT NULL,
    zid integer NOT NULL
);


ALTER TABLE public.page_ids OWNER TO postgres;

--
-- Name: participant_locations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.participant_locations (
    zid integer NOT NULL,
    uid integer NOT NULL,
    pid integer NOT NULL,
    lat double precision NOT NULL,
    lng double precision NOT NULL,
    created bigint DEFAULT public.now_as_millis(),
    source integer NOT NULL
);


ALTER TABLE public.participant_locations OWNER TO postgres;

--
-- Name: participant_metadata_answers; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.participant_metadata_answers (
    pmaid integer NOT NULL,
    pmqid integer,
    zid integer,
    value character varying(999),
    alive boolean DEFAULT true,
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.participant_metadata_answers OWNER TO postgres;

--
-- Name: participant_metadata_answers_pmaid_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.participant_metadata_answers_pmaid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.participant_metadata_answers_pmaid_seq OWNER TO postgres;

--
-- Name: participant_metadata_answers_pmaid_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.participant_metadata_answers_pmaid_seq OWNED BY public.participant_metadata_answers.pmaid;


--
-- Name: participant_metadata_choices; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.participant_metadata_choices (
    zid integer,
    pid integer,
    pmqid integer,
    pmaid integer,
    alive boolean DEFAULT true,
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.participant_metadata_choices OWNER TO postgres;

--
-- Name: participant_metadata_questions; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.participant_metadata_questions (
    pmqid integer NOT NULL,
    zid integer,
    key character varying(999),
    alive boolean DEFAULT true,
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.participant_metadata_questions OWNER TO postgres;

--
-- Name: participant_metadata_questions_pmqid_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.participant_metadata_questions_pmqid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.participant_metadata_questions_pmqid_seq OWNER TO postgres;

--
-- Name: participant_metadata_questions_pmqid_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.participant_metadata_questions_pmqid_seq OWNED BY public.participant_metadata_questions.pmqid;


--
-- Name: participants; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.participants (
    pid integer NOT NULL,
    uid integer NOT NULL,
    zid integer NOT NULL,
    vote_count integer DEFAULT 0 NOT NULL,
    last_interaction bigint DEFAULT 0 NOT NULL,
    subscribed integer DEFAULT 0 NOT NULL,
    last_notified bigint DEFAULT 0,
    nsli smallint DEFAULT 0 NOT NULL,
    mod integer DEFAULT 0 NOT NULL,
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.participants OWNER TO postgres;

--
-- Name: participants_extended; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.participants_extended (
    uid integer NOT NULL,
    zid integer NOT NULL,
    referrer character varying(9999),
    parent_url character varying(9999),
    created bigint DEFAULT public.now_as_millis(),
    modified bigint DEFAULT public.now_as_millis() NOT NULL,
    subscribe_email character varying(256),
    show_translation_activated boolean,
    permanent_cookie character varying(32),
    origin character varying(9999)
);


ALTER TABLE public.participants_extended OWNER TO postgres;

--
-- Name: permanentcookiezidjoins; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.permanentcookiezidjoins (
    zid integer NOT NULL,
    cookie character varying(32),
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.permanentcookiezidjoins OWNER TO postgres;

--
-- Name: pwreset_tokens; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.pwreset_tokens (
    uid integer,
    created bigint DEFAULT public.now_as_millis(),
    token character varying(250)
);


ALTER TABLE public.pwreset_tokens OWNER TO postgres;

--
-- Name: report_comment_selections; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.report_comment_selections (
    zid integer NOT NULL,
    rid bigint NOT NULL,
    tid integer NOT NULL,
    selection smallint NOT NULL,
    modified bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.report_comment_selections OWNER TO postgres;

--
-- Name: reports; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.reports (
    rid bigint NOT NULL,
    report_id character varying(300) NOT NULL,
    zid integer NOT NULL,
    created bigint DEFAULT public.now_as_millis(),
    modified bigint DEFAULT public.now_as_millis(),
    report_name character varying(999),
    label_x_neg character varying(999),
    label_x_pos character varying(999),
    label_y_neg character varying(999),
    label_y_pos character varying(999),
    label_group_0 character varying(999),
    label_group_1 character varying(999),
    label_group_2 character varying(999),
    label_group_3 character varying(999),
    label_group_4 character varying(999),
    label_group_5 character varying(999),
    label_group_6 character varying(999),
    label_group_7 character varying(999),
    label_group_8 character varying(999),
    label_group_9 character varying(999)
);


ALTER TABLE public.reports OWNER TO postgres;

--
-- Name: reports_rid_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.reports_rid_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.reports_rid_seq OWNER TO postgres;

--
-- Name: reports_rid_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.reports_rid_seq OWNED BY public.reports.rid;


--
-- Name: site_domain_whitelist; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.site_domain_whitelist (
    site_id character varying(256) NOT NULL,
    domain_whitelist character varying(999),
    domain_whitelist_override_key character varying(999),
    modified bigint DEFAULT public.now_as_millis() NOT NULL,
    created bigint DEFAULT public.now_as_millis() NOT NULL
);


ALTER TABLE public.site_domain_whitelist OWNER TO postgres;

--
-- Name: social_settings; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.social_settings (
    uid integer NOT NULL,
    polis_pic character varying(3000)
);


ALTER TABLE public.social_settings OWNER TO postgres;

--
-- Name: stars; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.stars (
    zid integer NOT NULL,
    pid integer NOT NULL,
    tid integer NOT NULL,
    starred integer NOT NULL,
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.stars OWNER TO postgres;

--
-- Name: suzinvites; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.suzinvites (
    owner integer NOT NULL,
    zid integer NOT NULL,
    xid character varying(32) NOT NULL,
    created bigint DEFAULT public.now_as_millis(),
    suzinvite character varying(32)
);


ALTER TABLE public.suzinvites OWNER TO postgres;

--
-- Name: trashes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.trashes (
    zid integer NOT NULL,
    pid integer NOT NULL,
    tid integer NOT NULL,
    trashed integer NOT NULL,
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.trashes OWNER TO postgres;

--
-- Name: twitter_users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.twitter_users (
    uid integer NOT NULL,
    twitter_user_id bigint NOT NULL,
    screen_name character varying(999) NOT NULL,
    name character varying(9999),
    followers_count integer NOT NULL,
    friends_count integer NOT NULL,
    verified boolean NOT NULL,
    profile_image_url_https character varying(9999),
    location character varying(9999),
    response json,
    modified bigint DEFAULT public.now_as_millis() NOT NULL,
    created bigint DEFAULT public.now_as_millis() NOT NULL
);


ALTER TABLE public.twitter_users OWNER TO postgres;

--
-- Name: upvotes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.upvotes (
    uid integer,
    zid integer
);


ALTER TABLE public.upvotes OWNER TO postgres;

--
-- Name: users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.users (
    uid integer NOT NULL,
    hname character varying(746),
    created bigint DEFAULT public.now_as_millis(),
    username character varying(128),
    email character varying(256),
    is_owner boolean DEFAULT false,
    zinvite character varying(300),
    oinvite character varying(300),
    tut smallint DEFAULT 0,
    site_id character varying(256) DEFAULT public.random_polis_site_id() NOT NULL,
    site_owner boolean DEFAULT true
);


ALTER TABLE public.users OWNER TO postgres;

--
-- Name: users_uid_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.users_uid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.users_uid_seq OWNER TO postgres;

--
-- Name: users_uid_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.users_uid_seq OWNED BY public.users.uid;


--
-- Name: votes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.votes (
    zid integer NOT NULL,
    pid integer NOT NULL,
    tid integer NOT NULL,
    vote smallint,
    weight_x_32767 smallint DEFAULT 0,
    created bigint DEFAULT public.now_as_millis(),
    high_priority boolean DEFAULT false NOT NULL
);


ALTER TABLE public.votes OWNER TO postgres;

--
-- Name: votes_latest_unique; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.votes_latest_unique (
    zid integer NOT NULL,
    pid integer NOT NULL,
    tid integer NOT NULL,
    vote smallint,
    weight_x_32767 smallint DEFAULT 0,
    modified bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.votes_latest_unique OWNER TO postgres;

--
-- Name: worker_tasks; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.worker_tasks (
    created bigint DEFAULT public.now_as_millis(),
    math_env character varying(999) NOT NULL,
    attempts smallint DEFAULT 0 NOT NULL,
    task_data jsonb NOT NULL,
    task_type character varying(99),
    task_bucket bigint,
    finished_time bigint
);


ALTER TABLE public.worker_tasks OWNER TO postgres;

--
-- Name: xid_whitelist; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.xid_whitelist (
    owner integer NOT NULL,
    xid text NOT NULL,
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.xid_whitelist OWNER TO postgres;

--
-- Name: xids; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.xids (
    uid integer NOT NULL,
    owner integer NOT NULL,
    xid text NOT NULL,
    x_profile_image_url character varying(3000),
    x_name character varying(746),
    x_email character varying(256),
    created bigint DEFAULT public.now_as_millis(),
    modified bigint DEFAULT public.now_as_millis() NOT NULL
);


ALTER TABLE public.xids OWNER TO postgres;

--
-- Name: zinvites; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.zinvites (
    zid integer NOT NULL,
    zinvite character varying(300) NOT NULL,
    created bigint DEFAULT public.now_as_millis()
);


ALTER TABLE public.zinvites OWNER TO postgres;

--
-- Name: contexts context_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.contexts ALTER COLUMN context_id SET DEFAULT nextval('public.contexts_context_id_seq'::regclass);


--
-- Name: conversations zid; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.conversations ALTER COLUMN zid SET DEFAULT nextval('public.conversations_zid_seq'::regclass);


--
-- Name: courses course_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.courses ALTER COLUMN course_id SET DEFAULT nextval('public.courses_course_id_seq'::regclass);


--
-- Name: participant_metadata_answers pmaid; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participant_metadata_answers ALTER COLUMN pmaid SET DEFAULT nextval('public.participant_metadata_answers_pmaid_seq'::regclass);


--
-- Name: participant_metadata_questions pmqid; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participant_metadata_questions ALTER COLUMN pmqid SET DEFAULT nextval('public.participant_metadata_questions_pmqid_seq'::regclass);


--
-- Name: reports rid; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.reports ALTER COLUMN rid SET DEFAULT nextval('public.reports_rid_seq'::regclass);


--
-- Name: users uid; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users ALTER COLUMN uid SET DEFAULT nextval('public.users_uid_seq'::regclass);


--
-- Data for Name: apikeysndvweifu; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.apikeysndvweifu (uid, apikey, created) FROM stdin;
\.


--
-- Data for Name: auth_tokens; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.auth_tokens (token, uid, created) FROM stdin;
U7eKUuG1BDCtuqlCgq9Q	2	1736644681465
2AqYb1l9t1mhoErmHP4Z	3	1736648226801
fFhHrjPAL78GCUYgFM2p	4	1736651755443
4cS8MfFKtdmuOnUJ61cB	5	1736651769851
aZcdITMCcmMYXPt44G5i	6	1736651789083
O6cY54IAZQRd0yYPnVJP	7	1736651807266
hAwDUJn8uHwgg49nyoib	8	1736651833939
F1PqXeeLARVlDNsdIOD6	9	1736652321512
VVxn1IMCLLfbYO7rI2xF	10	1736652375059
dWkZutEsQazdAreQYzB4	11	1736653653720
nMQ1NgS2EpmEEjMxJoPP	12	1736653654756
ntO9qITAvT0LQTSItoKJ	13	1736654381036
IXAfYpYbtXsLZTMsxz4Z	14	1736654517917
WRcn2NaJgE23rktA6ETm	15	1736654561419
ch7gEZ4VPvXJeYFykh8U	16	1736654654161
Ty2bWsJ7wbOvN1i9lmRu	17	1736654747965
2RK58TNIKmm8sCKfthm7	18	1736654854110
BbWLfBAOBMxTVv5DNPrQ	19	1736654927196
BuSEsaA2g96LKxKpKQoK	20	1736655204351
wAOX3MKkaFlqcYiqjFyr	21	1736655285246
oW2OuIMuCccXwT03VQfD	22	1736655354506
66ji0yIOGKEU6dLLFMhA	23	1736656290984
CfQ4sFzhFbw1vJ6fftXb	24	1736656341026
7YacvVPhDBmFIv4x4mbt	25	1736656787597
ZH3iwjEZOnqXoKVQFkFc	26	1736657017423
LvVEsYRFQfr6kHBPNNTm	27	1736657028349
uRq96dZ6AHkxmfyaLFG8	28	1736657159874
gpK04VWOZstVIlsPgmoY	29	1736657170162
DmvNVBQN0MDTza66dNNT	30	1736657185850
5t6uPo3n5FREPaWtswDY	31	1736657209875
EM6EL4yYXFKodIegZpcO	32	1736657211852
UGYDSEsIaifjengJv5Ny	33	1736657594910
C6Ra18wjEgppppjgF3vN	34	1736657746082
P3gbxvPvjg7dnxxzDiO4	35	1736657793174
gOVZExYLGMw9TCE0CSXU	36	1736657847056
nzEbka83cw3klMBFCiz9	37	1736657926183
pWn1Pfq9kRxWF6AyhlSC	38	1736658160246
ogGor28xCx5KpcDzVn34	39	1736663252728
axy8rx13oQU57TOW1jRz	40	1736664442693
f5QLfV6OMrPf0LRQjvGA	41	1736666020887
orZTB3md3uZnZHyzV0U3	42	1736669336051
2QX0kI2sf2Vq8SUihbrD	43	1736673036842
59r6psd6CmtgnG91pfnI	44	1736678082186
iaDBAs8wVQMU44voE9CE	45	1736679613964
8SaUUCfstDXVhUOTN2em	46	1736690333383
z7IZb3nfLuHHxnngXeuT	47	1736696208656
RSQCWqF6HZlAhsLq7IyW	48	1736699354391
v5cMfuhwGEE6K1qW9MCc	49	1736699599406
k1REHt62IjJLk0TXInPp	50	1736699755029
hvKwDJTzGJZYZPt3MjkS	51	1736699797643
cOdIs6D06WBECZ3AD4YY	52	1736699984860
09v5UwT6Azk1oacY2jyi	53	1736700557247
JrsewhyGGpjSRTE3iUgX	54	1736700869722
KBe7nqUXarJxMbRzNC4a	55	1736701402267
AFPQ0E6c6AE19nerUXr6	56	1736701484304
bm5Fnc9uXwfb8rLXE4fn	57	1736701502188
pla5ME1k5AbYhKD9ZWo0	58	1736701670329
lV2xho8KeRzaZVJhTSKc	59	1736701789092
ZcHMtoU68erH0N71GCPp	60	1736702009804
HM5bxIWeMAB7Skrri5KM	61	1736702311354
cclxcrGE9Yhr148IygoK	62	1736703188161
41889AZEl8pVXycqpMmt	63	1736703594976
QVr0RkfczgUZjhapg9Cw	64	1736708201388
RIIeW93AqkMVUoNI3TTf	65	1736708752137
49jgAhHP9LWNuqOyMlei	66	1736717867250
o5s8K3k9zJ9vvtCLnUfT	67	1736719950167
YyGWGZHVj6T33amzYjxu	68	1736732431745
EDDjtfG7iPqVvxgyZUUz	69	1736734107408
2pkuRbCVPabtkyF7IroM	70	1736734137885
exyJ1Obxfzl494TofQpe	71	1736734372911
nRHa3KepX1RKUwFLxFVI	72	1736734393736
UlrUos2CGjRcJIE555vx	73	1736734566259
Tl9VBc3FCeYx6Fnij7k1	74	1736735101021
NW7JgSgROwYhji3eES0i	75	1736735626101
IN0BMp8LCnWiBO7RyJtg	76	1736735670821
M6VizsuvcvClEO5T0weJ	77	1736735704374
bwQKY5Nxyt2Nl3LIHRdv	78	1736736427801
WrZdDTvjNDK7ajv7ahFC	79	1736736463981
PvPpOiVBsxFLJORQZJSW	80	1736736532709
48E7ZCLUbtPPXlHtloRQ	81	1736736989006
wRkYVBabm2kVKfJExbva	82	1736738953557
KowmNQMf0xxgd3qoEZje	83	1736740161586
Agb0f7bVQcwmN4Eyy6OZ	84	1736742667010
bdtgXEVTg2Fk3NQYosMk	85	1736744241282
MQqp99zFYxNnwDeCZdnx	86	1736756067746
pvl2Z6D6pDPjvdTG1wIs	87	1736794349391
DyO6VVMD0brVgPRxMd1e	88	1736794459140
mpbbUh9OFirlYC8SYFce	89	1736794490249
59NoecNqwHwKlsH0zKFQ	90	1736794871116
z4XpWdQ6RurLGjqySjOa	91	1736795450833
fXBsvWARxbce4dXYPLvf	92	1736795556232
PkKxfbJz0mjPGF8dF0ii	93	1736795556730
jc3JT7JzggEpHHMVYR3z	94	1736795587104
id77zd0M8g4aBNDTl1Ul	95	1736797828976
Qak1gZ7po0ZWzvhjm8re	96	1736801498995
enD2M7dLxCwDQGti7bSn	97	1736802782328
NAL2tvCCjbngTGdKJnQk	98	1736803832838
eFuJv177g0iMDU0Cg805	99	1736804657900
OsHxvGCBB4poGStV2AfE	100	1736809741521
1wOKgKlCH6A6bpAa2huu	101	1736814068058
B61cYXGh4TZtvhvOGKCd	102	1736814214345
sCdD074VyU3yv9wAlOOi	103	1736814454234
wvggtxB4waFGwVRwPsZi	104	1736826738909
9ttNtLtX4lmr9HTKt4Rz	105	1736879781062
IX2p3rLeoY3lv2BE7DL7	106	1736879802965
EgN2Iz7o0BtbqDKK7joR	107	1736879876159
fSkGd6BLuUAju3UeU6UO	108	1736880375709
AL3GWTu6OOc4RpBOqVVc	109	1736880472146
GFQIMUossEw1dEMTD0dl	110	1736880731766
zzohHAeBx5b6q8F21cJm	111	1736881628244
nrXtWiEElzIB7du00nk4	112	1736882248286
dCzYZumj6yge3OpcYyyI	113	1736916054099
3PqQL2j6kejiVNArgLKU	114	1736958241110
oj0LVqvj5eusrduJa7LI	115	1737002015142
op3TS6ofElnQUGBQl3rp	116	1737041230330
cSRNpi3BtLLXBjSbM77u	117	1737265238134
tw5bJ4rWK7AdqwjulB7V	118	1737467377031
6ZkEQKqzYL6txv5UJ0rd	119	1737730363455
kZoAu7ThNJIruxTL6GAG	120	1737739896610
k5w7LuN8vxS0TnwXoddG	121	1738358696899
rCVlHvnuwoGYnpZ8ZXU2	122	1739296854871
tmYmIdKHw8GEC84gBQyi	124	1739296921419
ITqmvyIvwRfODrGdgGnZ	125	1739296933266
rDR1lqHDJ4PBqMx2z8WH	126	1739296942894
XCzDzmM21lK1u74xLM5O	127	1739296950346
JpLdkBd7mQsm8x1eEB8s	128	1739296959209
i3tJ6myruclq33OzScIL	129	1739296969736
gkTPSLnvKNnfKbrFszso	130	1739296989744
2nrIIgnmr9x5iUflukgf	131	1739296989895
LEAd7UnJCreCpNon0Eha	122	1739310082708
bA5i24knJpl01DTTg0fu	122	1739312307452
awHK6lK9emzvjWZr8JKN	172	1739345825830
uRF4oYDKiAcbPuVpyToH	122	1739353316923
zeDA9wDcSlvzfYt8PlrI	175	1739371790516
Dhf2N0x2G83L4LwLkRDZ	177	1739381298404
mfdxi2w63xw68I4VafKc	178	1739384417603
sKsRCCsxG3rpuroI0ij4	179	1739453811209
6nRSDFo4V8CdFQ7Wae9o	180	1739453994076
zAym03Ojvk550NyOiTQN	182	1739454374635
aXHkSn8ImbZPr5SSpm4a	122	1739454937524
XqEk7FFZPSa0ZA8d0agy	209	1739455389380
GMEkTRcomcJm7H8l2ISp	215	1739457806401
JOV970jMNsy8Hur77xg5	228	1739460658366
vu4IKetsxAjM4EfIb828	235	1739462747979
veqjyc45SeMrqFDXoNPe	244	1739539452234
8xn5oDvxyZsJ6vDCkguz	246	1739735279351
\.


--
-- Data for Name: beta; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.beta (name, email, organization, created) FROM stdin;
\.


--
-- Data for Name: comment_translations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.comment_translations (zid, tid, src, txt, lang, created, modified) FROM stdin;
3	5	-1	Meow	en	1736689224889	1736689224889
3	4	-1	Small	en	1736689288613	1736689288613
3	3	-1	Ouah ouah	fr	1736689393762	1736689393762
3	3	-1	Woop Wooop	en	1736689420507	1736689420507
2	32	-1	Build out distributed technical infrastructure to support atomized & networked local organizing and action.	en	1736690318556	1736690318556
2	53	-1	No one should be called into work while this emergency is ongoing.	en	1736690333490	1736690333490
2	30	-1	I'm confident that I know where to direct my time/resources in this emergency	en	1736690337853	1736690337853
2	6	-1	I had to evacuate from my home.	en	1736690345001	1736690345001
2	34	-1	To commit to community care is inherently political. All people should recognize that the state seeks to prohibit community care.	en	1736690349635	1736690349635
2	55	-1	On the ground roles and remote roles are equally valued and critical in any organizational effort.	en	1736690357168	1736690357168
2	48	-1	Entirely flat organization structures result in a diffusion of responsibility and poorly coordinated responses.	en	1736690362280	1736690362280
2	2	-1	I witnessed police actively disrupting the distribution of aid.	en	1736690372739	1736690372739
2	54	-1	We should create roles and spaces for disabled folks to participate even if they cannot participate in person.	en	1736690380676	1736690380676
2	49	-1	Our collective mutual aid response to the fires as of Saturday night has been efficient.	en	1736690384642	1736690384642
2	43	-1	We need to organize around LandBack.	en	1736690396419	1736690396419
2	39	-1	Fire relief efforts have shown me it's not as difficult to organize in-person collective action in this city as I thought it was.	en	1736690399521	1736690399521
2	1	-1	I have lost my home in the fire.	en	1736690533288	1736690533288
2	42	-1	We must insure inclusive preparedness for future disasters to make sure Disabled people are cared for.	en	1736690536735	1736690536735
2	33	-1	There is no reform in a system that was built to specifically disempower the people. Only total reconstruction will ensure survival.	en	1736690542657	1736690542657
2	50	-1	Our collective mutual aid response to the fires as of Saturday night has been effective.	en	1736690550059	1736690550059
2	10	-1	The city/county should put a moratorium on evictions.	en	1736696219647	1736696219647
2	38	-1	I am participating on the ground at distro sites.	en	1736696241745	1736696241745
2	36	-1	Participating in these efforts has led me to want to be more engaged in addressing issues that affect our whole community.	en	1736696247323	1736696247323
2	44	-1	We should focus on returning lands to indigenous stewardship.	en	1736696251204	1736696251204
2	37	-1	I don't think mutual aid organizing can replace the government entirely without more sustained community organizing.	en	1736696253703	1736696253703
2	28	-1	We should call on the city to turn down the 2028 bid for the Olympics to prioritize resources for reconstruction efforts for local Angelenos	en	1736696258395	1736696258395
2	56	-1	we can figure out how to get infrastructure  stewarded by the state into a different form of stewardship hands of mutual aid organizers	en	1736699084137	1736699084137
2	51	-1	Marginalized people are being left out of our mutual aid efforts.	en	1736699362550	1736699362550
2	52	-1	I feel like I am making lasting relationships with people I am interacting with while doing mutual aid.	en	1736699364808	1736699364808
2	47	-1	We should make sure ecologically sound building methods with better fire resistance like cob and superadobe are prioritized for rebuilding	en	1736699411469	1736699411469
2	21	-1	We can do more than fill in gaps left by the state, we can build our own alternatives.	en	1736699649178	1736699649178
2	40	-1	We should demand the city divert LAPD funding to climate mitigation efforts.	en	1736699653994	1736699653994
2	27	-1	We can advocate for policy changes within the state while building infrastructure outside the state.	en	1736699659343	1736699659343
2	45	-1	Developing easily shared best practices and protocols for organizing the flow of resources would reduce the overwhelm.	en	1736699726523	1736699726523
2	46	-1	We must center our most vulnerable communities – disabled, trans, Black and indigenous, unhoused, undocumented, etc — in plans and action.	en	1736699806565	1736699806565
2	35	-1	The government and politicians will never save us. Policy is what led our planet to climate crisis and therefore these fire storms.	en	1736699818423	1736699818423
2	15	-1	We should keep up free food distribution beyond the immediate crisis.	en	1736699892239	1736699892239
2	12	-1	We need to organize and sustain community run health clinics.	en	1736699904702	1736699904702
2	31	-1	This has shown we need to stop waiting for permission or for someone else to save us. It's in our hands, it's always been in our hands.	en	1736699913101	1736699913101
2	11	-1	We need to stop building in the wildland urban interface.	en	1736699932647	1736699932647
2	41	-1	Attempting to replace police state without having some level engagement in existing civic structure will only result in militarized repression of popular movements.	en	1736699946770	1736699946770
2	13	-1	We cannot rely on local government, we need to make our own systems of care.	en	1736699950062	1736699950062
2	3	-1	The city should reallocate budget from police to better serve the community.	en	1736699968716	1736699968716
2	9	-1	The city/county should freeze rents.	en	1736699979013	1736699979013
2	25	-1	I think we need more community organized political education.	en	1736699988251	1736699988251
2	20	-1	We can build our own systems of care while pushing for reform.	en	1736700003090	1736700003090
2	23	-1	We can communicate a need for reform while we build an alternative.	en	1736700006747	1736700006747
2	5	-1	We should push the city to make policy changes.	en	1736700020241	1736700020241
2	22	-1	Spending energy on reform isn't helpful to our causes.	en	1736700028313	1736700028313
2	24	-1	There are enough political groups already working for change, we should focus our efforts on encouraging people to join those.	en	1736700041592	1736700041592
2	17	-1	The city should not sweep encampments during a state of emergency.	en	1736700046330	1736700046330
2	29	-1	We should adopt a "yes and" approach to solutions, instead of arguing about what's ideal. Try everything. Reduce harm. Fight on every front.	en	1736700048513	1736700048513
2	57	-1	reforms not reformism	en	1736701818039	1736701818039
2	58	-1	the best organizing methods are autonomous, outside and against the state. inside/outside is a failed strategy.	en	1736701887591	1736701887591
2	59	-1	We need more politicization in our mutual aid efforts. Otherwise, we won’t build with the community + will repeat this cycle over and over.	en	1736702456023	1736702456023
2	60	-1	Form community disaster response groups and study existing disaster response curricula. Adapt to your goals.	en	1736702743768	1736702743768
2	58	-1	最好的组织方法是自主的、外部的和反对国家的。内部/外部是一种失败的策略。	zh	1736703787083	1736703787083
2	61	-1	Demand 20,000 FEMA trailers for the new unhoused.	en	1736705581166	1736705581166
2	62	-1	Our volunteers sorting out inventory know how to best organize items for direct distributions to families/unhoused/ppl w-specfic needs etc	en	1736708907340	1736708907340
2	63	-1	We need better education about dealing with the toxic ash.	en	1736709161295	1736709161295
2	64	-1	I have not seen enough concern for disabled, chronically ill, and immunocompromised folx. They get left behind during emergencies	en	1736720783223	1736720783223
2	65	-1	I have not seen enough work to involved disabled folx in organizing - they are vital contributors to our movement.	en	1736720791631	1736720791631
2	66	-1	We need to focus more efforts on checking on and helping disabled folx and the elderly.	en	1736724715996	1736724715996
2	68	-1	The only reason why PPE and air filters are available now is bc covid activists continued to advocate for clean air/mask since 2020.	en	1736724723164	1736724723164
2	67	-1	We need to commit to normalizing PPE and wearing N95s - NOT just when protesting, but always. Disabled folks have warned us -we need 2listen	en	1736724728483	1736724728483
2	69	-1	I know who to organize with after the surge of mutual aid will subside after this calamity.	en	1736734566595	1736734566595
2	70	-1	I know where to go to continue having political discussions over the future of my community.	en	1736734581529	1736734581529
2	71	-1	To inspire homeowners NOT to rebuild, the city, county, or state should buy the land so they can relocate to a safer area. Rewild!	en	1736736265532	1736736265532
2	72	-1	Demand FEMA trailers for all unhoused, whether newly or not.	en	1736736547510	1736736547510
2	73	-1	During the rebuilding efforts, we should advocate for neighborhood land trusts	en	1736743875841	1736743875841
4	1	-1	The building I live in has burned down completely.	en	1736756050805	1736756050805
4	0	-1	I have a home or garden that has been affected by the fires.	en	1736756067823	1736756067823
4	5	-1	I don't feel like I have enough information about how to stay safe from the toxic ashes	en	1736756098418	1736756098418
4	3	-1	I have questions about cleaning up the toxic ash in the soil.	en	1736756101163	1736756101163
4	4	-1	I'm worried about what contaminants might be in the soil.	en	1736756104333	1736756104333
4	6	-1	I want to grow food in my garden as soon as possible	en	1736756107399	1736756107399
4	2	-1	I live in an area that was within the fire perimeter.	en	1736756110752	1736756110752
3	2	-1	Hey I am here	en	1736767955467	1736767955467
5	12	-1	I organize in SGV area.	en	1736794307523	1736794307523
5	9	-1	I organize in Central LA	en	1736794349493	1736794349493
5	6	-1	We should organize in person de-briefs with the liberal leaning orgs and spaces who capitulated when the state shut down mutual aid operations. Educate about our political analysis.	en	1736794351692	1736794351692
5	11	-1	I organize in SFV area.	en	1736794358670	1736794358670
5	1	-1	I organize in northeast LA.	en	1736794360613	1736794360613
5	10	-1	I organize in South LA	en	1736794365819	1736794365819
5	5	-1	I think we should focus on political education and outreach.	en	1736794388814	1736794388814
5	8	-1	We should do an coordinated multichannel media effort focused on the failures of the state and the gaps in care still present	en	1736794395011	1736794395011
5	3	-1	I organize from a remote location.	en	1736794398872	1736794398872
5	2	-1	I organize in West LA and adjacent areas.	en	1736794401360	1736794401360
5	4	-1	I think we can focus our efforts on supporting folks who are neglected by the state, disabled, undocumented.	en	1736794403270	1736794403270
5	7	-1	We should host community care and processing events where we can do political education in the next few days. Broad outreach, attract less informed people who may be new to mutual aid.	en	1736794406949	1736794406949
5	13	-1	We should have canned, accessible political ed flyers/leaflets that any autonomous group can include in their distros	en	1736794491883	1736794491883
5	15	-1	We should establish periodic phone banking to follow up with aid recipients and volunteers to bolster operations and do ongoing political ed	en	1736794604284	1736794604284
5	14	-1	Its important to form a coherent and easily identifiable coalition org to consolidate organizers/communication and attract/educate “normies”	en	1736794792346	1736794792346
5	16	-1	I organize in Long Beach	en	1736795564962	1736795564962
5	18	-1	Be mutual aid case-workers, in which we maintain consistent follow up with affected individuals to uplift each other until we’re liberated.	en	1736795938237	1736795938237
5	17	-1	we should have a coordinated response/media blast to counter the co-optation of our efforts (by the state/institutions/corporations)	en	1736795951640	1736795951640
5	19	-1	hold 'press conferences' for broadcast on trusted platforms (power leftist IG users, podcasts, etc) to put our analysis/experience out	en	1736796233169	1736796233169
5	20	-1	organize & empower those who receive mutual aid to become a part of the organizing	en	1736797012438	1736797012438
5	21	-1	run community health clinics	en	1736808494963	1736808494963
5	22	-1	get some people who manage infrastructure, like sanitation, fire dept, on the same page as us	en	1736808499314	1736808499314
3	6	-1	I am the last of my kind	en	1736899967339	1736899967339
3	0	-1	questions one	en	1736927651379	1736927651379
3	1	-1	Another question	en	1736931345991	1736931345991
3	8	-1	I think wild fires are scary	en	1737732195164	1737732195164
11	1	-1	another comment	en	1739296914857	1739296914857
11	0	-1	First comment	en	1739296921493	1739296921493
11	2	-1	This is a comment from me!	en	1739302659917	1739302659917
11	3	-1	XID comment	en	1739308575794	1739308575794
11	4	-1	HI	en	1739308670121	1739308670121
11	4	-1	SALUT	fr	1739309154895	1739309154895
11	2	-1	C'est un commentaire de moi !	fr	1739309575337	1739309575337
11	5	-1	new	en	1739316187905	1739316187905
11	7	-1	This is a test	en	1739316244171	1739316244171
9	0	-1	The time for AI regulation is behind us, now we need to get employees on board so that no one is left behind	en	1739317219157	1739317219157
12	28	-1	Ceux qui s'en servent en secret ne devraient pas le dire pour continuer à profiter du gain pour eux...	fr	1739455279001	1739455279001
9	1	-1	Let us be vigilant about the sovereignty of French-speaking data and its intellectual property in a deregulated generative world	en	1739317236268	1739317236268
11	8	-1	THis is a test	en	1739318757231	1739318757231
11	6	-1	New New	en	1739318773715	1739318773715
9	1	-1	Soyons vigilants à la souveraineté de la donnée francophone, et à sa propriété intellectuelle dans un monde génératif dérégulé	fr	1739345768993	1739345768993
9	0	-1	Le temps de la régulation IA est derrière nous, désormais il faut embarquer les salariés pour qu’aucun ne reste sur le bord de la route	fr	1739345825942	1739345825942
11	9	-1	Posting test	en	1739350926844	1739350926844
11	10	-1	Testing if the fix worked	en	1739362836812	1739362836812
11	11	-1	Another test	en	1739363105506	1739363105506
11	12	-1	test fffff	en	1739363141662	1739363141662
9	2	-1	A moratorium on generative AI should be proposed until its harmful effects on employees are brought under control	en	1739378652208	1739378652208
9	4	-1	We need an ambitious European regulatory programme.	en	1739381732210	1739381732210
9	12	-1	We see in current examples that AI does not increase employment but destroys it.	en	1739381788660	1739381788660
9	9	-1	The hours saved through generative AI are a useless productivity drain because they are not reused.	en	1739381858902	1739381858902
12	1	-1	Let us be vigilant about the sovereignty of French-speaking data and its intellectual property in a deregulated generative world.	en	1739382013139	1739382013139
9	7	-1	There is a need to consider broader rights and protections regarding worker data.	en	1739382033138	1739382033138
12	8	-1	The hours saved through generative AI are a useless productivity drain because they are not reused.	en	1739382111475	1739382111475
12	9	-1	Tax incentives can be an employment policy instrument in the face of the deployment of generative AI.	en	1739382115944	1739382115944
12	10	-1	Regulation must become a support: too many contradictory regulations and unclear frameworks prevent scaling up.	en	1739382135352	1739382135352
12	6	-1	There is a need to consider broader rights and protections regarding worker data.	en	1739382153281	1739382153281
12	7	-1	The paradox is that the employee is in favor of choosing his generative AI uses, while refusing the AI imposed by the company.	en	1739382175098	1739382175098
12	4	-1	Increasing the AI impact: it is examining how work is organized, valued; how industries are structured.	en	1739382179064	1739382179064
12	5	-1	The devaluation of work involves reducing the work of operators to data and losing control over their collection.	en	1739382224053	1739382224053
9	10	-1	Tax incentives can be an employment policy instrument in the face of the deployment of generative AI.	en	1739382309021	1739382309021
12	23	-1	We should not be afraid of AI, but we should not be afraid of a co-construction social dialogue either.	en	1739382353768	1739382353768
12	27	-1	We must agree to let employees be the actors in their choices and their digital uses at work.	en	1739382428402	1739382428402
9	5	-1	Augmenter l'impact IA : c’est examiner comment le travail est organisé, valorisé ; comment les industries sont structurées.	fr	1739382451290	1739382451290
9	8	-1	Le paradoxe, c’est que le salarié est en faveur de choisir ses usages IA générative, tout en refusant les IA imposées par l’entreprise.	fr	1739382783251	1739382783251
9	11	-1	Regulation must become a support: too many contradictory regulations and unclear frameworks prevent scaling up.	en	1739382857342	1739382857342
9	10	-1	Les incitations fiscales peuvent être un instrument de politique d’emploi face au déploiement d’IA générative.	fr	1739382925904	1739382925904
9	8	-1	The paradox is that the employee is in favor of choosing his generative AI uses, while refusing the AI imposed by the company.	en	1739382942756	1739382942756
12	0	-1	The time for AI regulation is behind us, now we need to get employees on board so that no one is left behind.	en	1739448929573	1739448929573
9	29	-1	La ministre du travail qui parle de dialogue social, alors que les gouvernements récents ont plutôt laminé ces structures. Drole.	fr	1739453948174	1739453948174
12	0	-1	Le temps de la régulation IA est derrière nous, désormais il faut embarquer les salariés pour qu’aucun ne reste sur le bord de la route.	fr	1739454229307	1739454229307
12	4	-1	Augmenter l'impact IA : c’est examiner comment le travail est organisé, valorisé ; comment les industries sont structurées.	fr	1739454339254	1739454339254
9	30	-1	Après le test ?	fr	1739454374726	1739454374726
12	17	-1	After the digital divide, the AI divide will be even more divisive in organizations.	en	1739454483371	1739454483371
12	17	-1	Après la fracture numérique, la fracture IA va être encore plus clivante dans les organisations.	fr	1739454493281	1739454493281
9	31	-1	Si impact de l'IA est intensification et développement des inégalités, alors comment on régule les cadences ?	fr	1739454695278	1739454695278
12	17	-1	Dopo il divario digitale, il divario nell'intelligenza artificiale sarà ancora più divisivo nelle organizzazioni.	it	1739454705200	1739454705200
12	17	-1	继数字鸿沟之后，AI鸿沟在组织中的分裂将更加严重。	zh	1739454717783	1739454717783
12	4	-1	Aumentare l'impatto dell'IA: ciò significa esaminare il modo in cui il lavoro è organizzato e valorizzato; come sono strutturate le industrie.	it	1739454718326	1739454718326
12	0	-1	Il momento della regolamentazione dell'intelligenza artificiale è ormai alle nostre spalle, ora dobbiamo coinvolgere i dipendenti affinché nessuno venga lasciato indietro.	it	1739454730722	1739454730722
12	4	-1	增加人工智能的影响力：这意味着研究工作如何组织和评估；行业是如何构成的。	zh	1739454775151	1739454775151
12	0	-1	人工智能监管的时代已经过去，现在我们需要让员工参与进来，这样就不会有人掉队。	zh	1739454791144	1739454791144
12	19	-1	80% de transformation c’est d'accompagnement l'humain et seulement 20% est de la  technologie.	fr	1739454975159	1739454975159
12	19	-1	80% of transformation is human support and only 20% is technology.	en	1739454976834	1739454976834
12	23	-1	Il ne faut pas avoir peur de l'IA, mais il ne faut pas non plus avoir peur d’un dialogue social de co-construction.	fr	1739455030291	1739455030291
12	27	-1	Il faut accepter de laisser les salariés acteurs de leurs choix et de leurs usages numériques au travail.	fr	1739455087388	1739455087388
12	28	-1	Those who use it secretly should not say so in order to continue to enjoy the gain for themselves...	en	1739455249180	1739455249180
12	23	-1	Non dobbiamo aver paura dell'intelligenza artificiale, ma non dobbiamo aver paura neanche di un dialogo sociale co-costruito.	it	1739455483735	1739455483735
12	27	-1	Dobbiamo accettare di lasciare che i dipendenti siano attori delle loro scelte e del loro utilizzo digitale sul lavoro.	it	1739455506486	1739455506486
12	28	-1	Chi ne fa uso di nascosto non dovrebbe dirlo, per continuare a goderne il guadagno...	it	1739455517994	1739455517994
12	19	-1	L'80% della trasformazione è dovuto al supporto umano e solo il 20% alla tecnologia.	it	1739455531239	1739455531239
12	29	-1	The use of AI is a strong individual leap for the individual user, but a disappointing contribution for the collective.	en	1739455658843	1739455658843
12	29	-1	L'usage de l'IA est un sautien individuel fort pour l'utiliseur individuel, mais un apport décevant pour le collectif	fr	1739455668025	1739455668025
12	8	-1	Les heures gagnées grâce à l’IA générative sont une fuite de productivité inutile parce qu’elles ne sont pas réutilisées.	fr	1739455714686	1739455714686
12	29	-1	L'impiego dell'intelligenza artificiale rappresenta un grande passo avanti per il singolo utente, ma un contributo deludente per la collettività.	it	1739455828824	1739455828824
12	8	-1	Le ore risparmiate grazie all'intelligenza artificiale generativa rappresentano un inutile spreco di produttività perché non vengono riutilizzate.	it	1739455835642	1739455835642
12	6	-1	Il y a nécessité d'envisager des droits et des protections plus étendus en ce qui concerne les données des travailleurs.	fr	1739455916767	1739455916767
12	30	-1	La plupart du temps, un savoir collectif passé est abusivement utilisé par l'ia pour donner des ordres aux salariés et supprimer leur manager	fr	1739456064580	1739456064580
12	30	-1	Most of the time, past collective knowledge is abusively used by AI to give orders to employees and remove their manager.	en	1739456065377	1739456065377
12	11	-1	On voit dans les exemples actuels que l’Ia n’augmente pas l’emploi mais le détruit.	fr	1739456234416	1739456234416
12	11	-1	We see in current examples that AI does not increase employment but destroys it.	en	1739456236829	1739456236829
9	32	-1	Pour protéger l’emploi et les travailleurs, le périmètre d’action des agents IA doit être limité à certaines tâches.	fr	1739456350144	1739456350144
12	11	-1	Gli esempi attuali dimostrano che l'intelligenza artificiale non aumenta l'occupazione, ma la distrugge.	it	1739456418003	1739456418003
12	6	-1	È necessario prendere in considerazione diritti e tutele più ampi per quanto riguarda i dati dei lavoratori.	it	1739456435334	1739456435334
12	30	-1	Nella maggior parte dei casi, la conoscenza collettiva del passato viene sfruttata in modo improprio dall'intelligenza artificiale per impartire ordini ai dipendenti e rimuovere i loro responsabili.	it	1739456441981	1739456441981
9	34	-1	Pour sensibiliser en masse (dont les travailleurs) à l’IA et à ses enjeux, initier une émission sur les chaînes télévisées nationales.	fr	1739456780193	1739456780193
9	33	-1	Comme le nutri-score qui évalue la qualité des aliments que nous ingérons, mettre en place un indicateur pour les niveaux intrusifs des IA	fr	1739456787816	1739456787816
9	35	-1	Dans la continuité de ma proposition d’émission télévisée nationale,former aux fondamentaux et accompagner nos citoyens à sa prise en main.	fr	1739456794473	1739456794473
9	36	-1	Dans chaque entreprise, mettre en place cellule « Notre entreprise plus compétitive demain » Besoin de plus de caractères pour expliquer :)	fr	1739456937854	1739456937854
9	37	-1	Oui, faire que les gains de productivité grâce à l'IA soient redistribués aux salariés.	fr	1739457017673	1739457017673
12	7	-1	Le paradoxe, c’est que le salarié est en faveur de choisir ses usages IA générative, tout en refusant les IA imposées par l’entreprise.	fr	1739457513235	1739457513235
12	31	-1	For the introduction of AI not to be a punishment, each employee must have their own training plan and an understanding of its interest.	en	1739457733039	1739457733039
12	31	-1	Pour que l'introduction IA ne soit pas une punition, chaque salarié doit avoir son plan de formation et une compréhension de son intérêt	fr	1739457735152	1739457735152
9	38	-1	Proposer que les entreprises qui recourent à l'iA s'engagent à ne pas débaucher dans le temps de déploiement + X annees	fr	1739457821052	1739457821052
12	32	-1	Training experts means starting with basic activities and then doing more complex ones. With AI, does that disappear?	en	1739457995701	1739457995701
12	32	-1	Former des experts, c'est commencer par les activités basiques puis en faire de plus complexes. Avec l'IA, ça disparaît ?	fr	1739458098982	1739458098982
12	33	-1	AIs should pass a product conformance test before being introduced to the market. Like any product.	en	1739458497410	1739458497410
12	2	-1	We need an ambitious European development programme.	en	1739458565436	1739458565436
12	3	-1	We need an ambitious European regulatory programme.	en	1739458574961	1739458574961
12	3	-1	Nous avons besoin d’un programme européen ambitieux de régulation.	fr	1739458582924	1739458582924
12	2	-1	Nous avons besoin d’un programme européen ambitieux de développement.	fr	1739458588999	1739458588999
12	33	-1	Les IA devraient passer un test de conformité produit avant introduction sur le marché. Comme tout produit.	fr	1739458598468	1739458598468
12	34	-1	AI agents should not operate beyond defined tasks in order to protect workers	en	1739458745863	1739458745863
12	34	-1	Les agents IA ne doivent pas opérer au-delà de tâches déimitées afin de protéger les travailleurs	fr	1739458748045	1739458748045
12	13	-1	AI is more than ever the subject of public debate. It must become a democratic commons.	en	1739459413577	1739459413577
12	1	-1	Soyons vigilants à la souveraineté de la donnée francophone, et à sa propriété intellectuelle dans un monde génératif dérégulé.	fr	1739459427861	1739459427861
12	13	-1	L'IA fait plus que jamais l'objet de débat public. Elle doit devenir un commun démocratique.	fr	1739459429238	1739459429238
12	3	-1	Abbiamo bisogno di un ambizioso programma normativo europeo.	it	1739459628455	1739459628455
12	1	-1	Dobbiamo vigilare sulla sovranità dei dati francofoni e sulla loro proprietà intellettuale in un mondo generativo deregolamentato.	it	1739459642509	1739459642509
12	2	-1	Abbiamo bisogno di un ambizioso programma di sviluppo europeo.	it	1739459649096	1739459649096
12	33	-1	Le IA dovrebbero superare un test di conformità del prodotto prima di essere introdotte sul mercato. Come qualsiasi prodotto.	it	1739459668328	1739459668328
12	32	-1	Formare gli esperti significa iniziare con attività di base e poi dedicarsi ad attività più complesse. Con l'intelligenza artificiale, scompare?	it	1739459678737	1739459678737
12	39	-1	La tecnologia farà ciò che le chiederemo di fare. Possiamo impedire che accada il contrario.	it	1739466326793	1739466326793
12	31	-1	Affinché l'introduzione dell'intelligenza artificiale non si trasformi in una punizione, ogni dipendente deve avere un proprio piano di formazione e comprendere i suoi interessi.	it	1739459720600	1739459720600
12	34	-1	Gli agenti di intelligenza artificiale non dovrebbero operare oltre i compiti definiti per proteggere i lavoratori	it	1739459751521	1739459751521
12	13	-1	L'intelligenza artificiale è più che mai oggetto di dibattito pubblico. Deve diventare un bene comune democratico.	it	1739459755063	1739459755063
12	7	-1	Il paradosso è che il dipendente è favorevole a scegliere l'utilizzo dell'IA generativa, mentre rifiuta l'IA imposta dall'azienda.	it	1739459797272	1739459797272
9	36	-1	In each company, set up a cell "Our company more competitive tomorrow" Need more characters to explain :)	en	1739460640996	1739460640996
9	33	-1	Like the nutri-score that evaluates the quality of the food we eat, setting up an indicator for the intrusive levels of AI	en	1739460658520	1739460658520
9	32	-1	To protect jobs and workers, the scope of action of AI agents must be limited to certain tasks.	en	1739460669377	1739460669377
9	34	-1	To raise mass awareness (including among workers) about AI and its challenges, initiate a program on national television channels.	en	1739460677726	1739460677726
9	31	-1	If the impact of AI is the intensification and development of inequalities, then how do we regulate the pace?	en	1739460685058	1739460685058
9	38	-1	Propose that companies using AI commit to not poaching during the deployment period + X years	en	1739460714676	1739460714676
9	35	-1	In line with my proposal for a national television program, provide training in the fundamentals and support our citizens in taking charge of it.	en	1739460719275	1739460719275
9	37	-1	Yes, ensuring that productivity gains from AI are redistributed to employees.	en	1739460721468	1739460721468
9	30	-1	After the test?	en	1739460724793	1739460724793
9	29	-1	The Minister of Labour who talks about social dialogue, while recent governments have rather crushed these structures. Funny.	en	1739460728949	1739460728949
12	14	-1	La boussole IA est une manière de prioriser des projets, si et seulement si la techno s'avère véritablement nécessaire pour le bien commun.	fr	1739462519648	1739462519648
12	25	-1	Il faut mettre en place les conditions d'un réel dialogue social technologique au travail.	fr	1739462547145	1739462547145
12	14	-1	The AI compass is a way to prioritize projects, if and only if the technology is truly necessary for the common good.	en	1739462802888	1739462802888
12	25	-1	We need to put in place the conditions for a real technological social dialogue at work.	en	1739462812381	1739462812381
12	35	-1	Lorsqu'on parle d'IA générative, on évoque souvent la productivité, mais pas assez l'humain.	fr	1739463188333	1739463188333
12	35	-1	When we talk about generative AI, we often talk about productivity, but not enough about humans.	en	1739463242376	1739463242376
12	22	-1	Nous devons penser aux générations futures qui seront touchées par nos expériences avec l’IA incontrôlées aux conséquences considérables.	fr	1739463395540	1739463395540
12	22	-1	We must think about future generations who will be affected by our uncontrolled experiments with AI with far-reaching consequences.	en	1739463406211	1739463406211
12	20	-1	On certain themes, we will choose not to launch generative AI projects.	en	1739463629169	1739463629169
12	20	-1	Sur certains thèmes, on fera le choix de ne pas lancer des projets d’IA générative.	fr	1739463632313	1739463632313
12	36	-1	L'IA ne nous remplace pas mais elle nous REPLACE.	fr	1739464067912	1739464067912
12	20	-1	Su alcuni temi sceglieremo di non lanciare progetti di intelligenza artificiale generativa.	it	1739464130979	1739464130979
12	22	-1	Dobbiamo pensare alle generazioni future che saranno colpite dai nostri esperimenti incontrollati con l'intelligenza artificiale, con conseguenze di vasta portata.	it	1739464139284	1739464139284
12	36	-1	AI does not replace us, but it REPLACES us.	en	1739464143029	1739464143029
12	14	-1	La bussola dell'intelligenza artificiale è un modo per stabilire le priorità dei progetti, solo se la tecnologia è realmente necessaria per il bene comune.	it	1739464148771	1739464148771
12	35	-1	Quando parliamo di intelligenza artificiale generativa, spesso parliamo di produttività, ma non abbastanza degli esseri umani.	it	1739464158071	1739464158071
12	36	-1	L'intelligenza artificiale non ci sostituisce, ma ci SOSTITUISCE.	it	1739464174392	1739464174392
12	25	-1	Dobbiamo creare le condizioni affinché si realizzi un vero dialogo sociale tecnologico.	it	1739464190838	1739464190838
12	37	-1	We must have the choice to "opt out".	en	1739464789251	1739464789251
12	37	-1	Il faut que nous ayons le choix de nous "opt out".	fr	1739464806914	1739464806914
12	21	-1	Limiting yourself to early adopters to disseminate? You need to have the right transformation rate for each population in the organization.	en	1739464839289	1739464839289
12	21	-1	Se limiter aux early adopteurs pour diffuser? Il faut avoir la bonne cadence de transformation pour chaque population dans l'organisation.	fr	1739464854715	1739464854715
12	18	-1	C'est important de sensibiliser à la sécurité, transparence ; à la façon de traiter, utiliser les données quand on n'a pas culture de data.	fr	1739464887996	1739464887996
12	18	-1	It is important to raise awareness about security, transparency; how to process and use data when we do not have a data culture.	en	1739464903919	1739464903919
12	37	-1	Dobbiamo avere la possibilità di "rinunciare".	it	1739465053236	1739465053236
12	18	-1	È importante aumentare la consapevolezza in materia di sicurezza e trasparenza; su come elaborare e utilizzare i dati quando non si ha una cultura dei dati.	it	1739465059499	1739465059499
12	21	-1	Vuoi limitarti a trasmettere solo ai primi utenti? È necessario disporre del giusto tasso di trasformazione per ogni popolazione dell'organizzazione.	it	1739465074393	1739465074393
12	26	-1	Understanding AI is not limited to the acquisition of technical skills and questions the notion of expertise and management.	en	1739465503048	1739465503048
12	26	-1	Appréhender l'IA ne se limite pas à l'acquisition de compétences techniques et interroge la notion d'expertise et de management.	fr	1739465505536	1739465505536
12	38	-1	Plus nous avons d'IA, plus nous avons besoin d'intelligence émotionnelle.	fr	1739465924061	1739465924061
12	38	-1	The more AI we have, the more emotional intelligence we need.	en	1739465949851	1739465949851
12	39	-1	La technologie fera ce que nous lui demanderons de faire. Nous pouvons empêcher que ce soit l'inverse.	fr	1739466010013	1739466010013
12	39	-1	Technology will do what we ask it to do. We can prevent it from doing the opposite.	en	1739466011439	1739466011439
12	38	-1	Più intelligenza artificiale abbiamo, più intelligenza emotiva ci serve.	it	1739466320574	1739466320574
12	26	-1	Comprendere l'intelligenza artificiale non si limita all'acquisizione di competenze tecniche e mette in discussione i concetti di competenza e gestione.	it	1739466333065	1739466333065
12	40	-1	Il faut des syndicats plus forts et mieux formés.	fr	1739468072899	1739468072899
12	40	-1	We need stronger and better trained unions.	en	1739468105349	1739468105349
12	41	-1	rêvons que les salariés retrouvent la défense de leurs métiers grâce à l'IA.	fr	1739468203436	1739468203436
12	41	-1	Let's dream that employees will find the defense of their professions thanks to AI.	en	1739468234038	1739468234038
12	40	-1	Abbiamo bisogno di sindacati più forti e meglio formati.	it	1739468592347	1739468592347
\.


--
-- Data for Name: comments; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.comments (tid, zid, pid, uid, created, modified, txt, velocity, mod, lang, lang_confidence, active, is_meta, tweet_id, quote_src_url, anon, is_seed) FROM stdin;
0	2	0	2	1736645112971	1736645112971	A	1	-1	\N	\N	t	f	\N	\N	f	t
1	2	0	2	1736645326830	1736645326830	I have lost my home in the fire.	1	1	\N	\N	t	t	\N	\N	f	t
2	2	0	2	1736645419824	1736645419824	I witnessed police actively disrupting the distribution of aid.	1	1	\N	\N	t	t	\N	\N	f	t
5	2	0	2	1736645696268	1736645696268	We should push the city to make policy changes.	1	1	\N	\N	t	f	\N	\N	f	t
6	2	0	2	1736645712949	1736645712949	I had to evacuate from my home.	1	1	\N	\N	t	t	\N	\N	f	t
7	2	0	2	1736645758811	1736645758811	The city should freeze rents.	1	-1	\N	\N	t	f	\N	\N	f	t
8	2	0	2	1736645773714	1736645773714	The county should put a moratorium on evictions.	1	-1	\N	\N	t	f	\N	\N	f	t
9	2	0	2	1736645801404	1736645801404	The city/county should freeze rents.	1	1	\N	\N	t	f	\N	\N	f	t
10	2	0	2	1736645807868	1736645807868	The city/county should put a moratorium on evictions.	1	1	\N	\N	t	f	\N	\N	f	t
11	2	0	2	1736645836826	1736645836826	We need to stop building in the wildland urban interface.	1	1	\N	\N	t	f	\N	\N	f	t
12	2	0	2	1736645874934	1736645874934	We need to organize and sustain community run health clinics.	1	1	\N	\N	t	f	\N	\N	f	t
4	2	0	2	1736645674977	1736645674977	We need to build our own systems of care.	1	-1	\N	\N	t	f	\N	\N	f	t
13	2	0	2	1736646064876	1736646064876	We cannot rely on local government, we need to make our own systems of care.	1	1	\N	\N	t	f	\N	\N	f	t
15	2	0	2	1736646664406	1736646664406	We should keep up free food distribution beyond the immediate crisis.	1	1	\N	\N	t	f	\N	\N	f	t
16	2	0	2	1736647227825	1736647227825	The city is harming unhoused folks by sweeping encampments.	1	-1	\N	\N	t	f	\N	\N	f	t
17	2	0	2	1736647444981	1736647444981	The city should not sweep encampments during a state of emergency.	1	1	\N	\N	t	f	\N	\N	f	t
14	2	0	2	1736646075829	1736646075829	We can build our own systems while pushing for reform.	1	-1	\N	\N	t	f	\N	\N	f	t
20	2	0	2	1736647550791	1736647550791	We can build our own systems of care while pushing for reform.	1	1	\N	\N	t	f	\N	\N	f	t
0	3	0	1	1736648104606	1736648104606	questions one	1	1	\N	\N	t	f	\N	\N	f	t
1	3	1	3	1736648226877	1736648226877	Another question	1	1	\N	\N	t	f	\N	\N	f	f
19	2	0	2	1736647514703	1736647514703	We aren't doing enough mutual aid when there aren't emergencies.	1	-1	\N	\N	t	f	\N	\N	f	t
18	2	0	2	1736647466564	1736647466564	We should actively obstruct sweeps of encampments of unhoused people.	1	-1	\N	\N	t	f	\N	\N	f	t
21	2	0	2	1736649660246	1736649660246	We can do more than fill in gaps left by the state, we can build our own alternatives.	1	1	\N	\N	t	f	\N	\N	f	t
22	2	0	2	1736649697430	1736649697430	Spending energy on reform isn't helpful to our causes.	1	1	\N	\N	t	f	\N	\N	f	t
23	2	0	2	1736649752787	1736649752787	We can communicate a need for reform while we build an alternative.	1	1	\N	\N	t	f	\N	\N	f	t
24	2	0	2	1736649905747	1736649905747	There are enough political groups already working for change, we should focus our efforts on encouraging people to join those.	1	1	\N	\N	t	f	\N	\N	f	t
25	2	0	2	1736649945135	1736649945135	I think we need more community organized political education.	1	1	\N	\N	t	f	\N	\N	f	t
2	3	2	4	1736651755525	1736651755525	Hey I am here	1	0	\N	\N	t	f	\N	\N	f	f
3	3	3	5	1736651769932	1736651769932	Woop Wooop	1	0	\N	\N	t	f	\N	\N	f	f
4	3	4	6	1736651789165	1736651789165	Yar	1	0	\N	\N	t	f	\N	\N	f	f
5	3	5	7	1736651807342	1736651807342	Meow	1	0	\N	\N	t	f	\N	\N	f	f
6	3	6	8	1736651834047	1736651834047	I am the last of my kind	1	0	\N	\N	t	f	\N	\N	f	f
26	2	1	9	1736652321600	1736652321600	我好累	1	-1	\N	\N	t	f	\N	\N	f	f
3	2	0	2	1736645624649	1736645624649	The city should reallocate budget from police to better serve the community.	1	1	\N	\N	t	f	\N	\N	f	t
32	2	13	21	1736655563292	1736655563292	Build out distributed technical infrastructure to support atomized & networked local organizing and action.	1	1	\N	\N	t	f	\N	\N	f	f
27	2	11	19	1736655065833	1736655065833	We can advocate for policy changes within the state while building infrastructure outside the state.	1	1	\N	\N	t	f	\N	\N	f	f
28	2	10	18	1736655163859	1736655163859	We should call on the city to turn down the 2028 bid for the Olympics to prioritize resources for reconstruction efforts for local Angelenos	1	1	\N	\N	t	f	\N	\N	f	f
29	2	13	21	1736655435920	1736655435920	We should adopt a "yes and" approach to solutions, instead of arguing about what's ideal. Try everything. Reduce harm. Fight on every front.	1	1	\N	\N	t	f	\N	\N	f	f
40	2	0	2	1736658087561	1736658087561	We should demand the city divert LAPD funding to climate mitigation efforts.	1	1	\N	\N	t	f	\N	\N	f	t
31	2	13	21	1736655495144	1736655495144	This has shown we need to stop waiting for permission or for someone else to save us. It's in our hands, it's always been in our hands.	1	1	\N	\N	t	f	\N	\N	f	f
33	2	15	23	1736656492195	1736656492195	There is no reform in a system that was built to specifically disempower the people. Only total reconstruction will ensure survival.	1	1	\N	\N	t	f	\N	\N	f	f
34	2	15	23	1736656618746	1736656618746	To commit to community care is inherently political. All people should recognize that the state seeks to prohibit community care.	1	1	\N	\N	t	f	\N	\N	f	f
35	2	15	23	1736656776308	1736656776308	The government and politicians will never save us. Policy is what led our planet to climate crisis and therefore these fire storms.	1	1	\N	\N	t	f	\N	\N	f	f
36	2	0	2	1736657019897	1736657019897	Participating in these efforts has led me to want to be more engaged in addressing issues that affect our whole community.	1	1	\N	\N	t	f	\N	\N	f	t
37	2	0	2	1736657112061	1736657112061	I don't think mutual aid organizing can replace the government entirely without more sustained community organizing.	1	1	\N	\N	t	f	\N	\N	f	t
38	2	0	2	1736657364870	1736657364870	I am participating on the ground at distro sites.	1	1	\N	\N	t	f	\N	\N	f	t
39	2	0	2	1736657461724	1736657461724	Fire relief efforts have shown me it's not as difficult to organize in-person collective action in this city as I thought it was.	1	1	\N	\N	t	f	\N	\N	f	t
41	2	0	2	1736658171661	1736658171661	Attempting to replace police state without having some level engagement in existing civic structure will only result in militarized repression of popular movements.	1	1	\N	\N	t	f	\N	\N	f	t
30	2	12	20	1736655444564	1736655444564	I'm confident that I know where to direct my time/resources in this emergency	1	1	\N	\N	t	f	\N	\N	f	f
42	2	0	2	1736659014843	1736659014843	We must insure inclusive preparedness for future disasters to make sure Disabled people are cared for.	1	1	\N	\N	t	f	\N	\N	f	f
43	2	0	2	1736659124743	1736659124743	We need to organize around LandBack.	1	1	\N	\N	t	f	\N	\N	f	f
44	2	0	2	1736659147394	1736659147394	We should focus on returning lands to indigenous stewardship.	1	1	\N	\N	t	f	\N	\N	f	f
45	2	0	2	1736659968252	1736659968252	Developing easily shared best practices and protocols for organizing the flow of resources would reduce the overwhelm.	1	1	\N	\N	t	f	\N	\N	f	t
47	2	0	2	1736661684700	1736661684700	We should make sure ecologically sound building methods with better fire resistance like cob and superadobe are prioritized for rebuilding	1	1	\N	\N	t	f	\N	\N	f	t
46	2	13	21	1736660283925	1736660283925	We must center our most vulnerable communities – disabled, trans, Black and indigenous, unhoused, undocumented, etc — in plans and action.	1	1	\N	\N	t	f	\N	\N	f	f
48	2	31	39	1736663636879	1736663636879	Entirely flat organization structures result in a diffusion of responsibility and poorly coordinated responses.	1	1	\N	\N	t	f	\N	\N	f	f
49	2	31	39	1736663822224	1736663822224	Our collective mutual aid response to the fires as of Saturday night has been efficient.	1	1	\N	\N	t	f	\N	\N	f	f
50	2	31	39	1736663834224	1736663834224	Our collective mutual aid response to the fires as of Saturday night has been effective.	1	1	\N	\N	t	f	\N	\N	f	f
51	2	0	2	1736666547040	1736666547040	Marginalized people are being left out of our mutual aid efforts.	1	1	\N	\N	t	f	\N	\N	f	t
52	2	0	2	1736666930810	1736666930810	I feel like I am making lasting relationships with people I am interacting with while doing mutual aid.	1	1	\N	\N	t	f	\N	\N	f	t
53	2	34	42	1736669336132	1736669336132	No one should be called into work while this emergency is ongoing.	1	1	\N	\N	t	f	\N	\N	f	f
54	2	11	19	1736676325364	1736676325364	We should create roles and spaces for disabled folks to participate even if they cannot participate in person.	1	1	\N	\N	t	f	\N	\N	f	f
55	2	11	19	1736676364787	1736676364787	On the ground roles and remote roles are equally valued and critical in any organizational effort.	1	1	\N	\N	t	f	\N	\N	f	f
56	2	37	45	1736679614043	1736679614043	we can figure out how to get infrastructure  stewarded by the state into a different form of stewardship hands of mutual aid organizers	1	1	\N	\N	t	f	\N	\N	f	f
57	2	47	55	1736701815983	1736701815983	reforms not reformism	1	1	en	1	t	f	\N	\N	f	f
58	2	47	55	1736701878522	1736701878522	the best organizing methods are autonomous, outside and against the state. inside/outside is a failed strategy.	1	1	en	1	t	f	\N	\N	f	f
59	2	51	59	1736702429766	1736702429766	We need more politicization in our mutual aid efforts. Otherwise, we won’t build with the community + will repeat this cycle over and over.	1	1	en	1	t	f	\N	\N	f	f
60	2	52	60	1736702737527	1736702737527	Form community disaster response groups and study existing disaster response curricula. Adapt to your goals.	1	1	en	1	t	f	\N	\N	f	f
61	2	52	60	1736704545202	1736704545202	Demand 20,000 FEMA trailers for the new unhoused.	1	1	en	0.988918	t	f	\N	\N	f	f
62	2	56	64	1736708882057	1736708882057	Our volunteers sorting out inventory know how to best organize items for direct distributions to families/unhoused/ppl w-specfic needs etc	1	1	en	1	t	f	\N	\N	f	f
63	2	0	2	1736709159365	1736709159365	We need better education about dealing with the toxic ash.	1	1	en	1	t	f	\N	\N	f	t
64	2	59	67	1736720059479	1736720059479	I have not seen enough concern for disabled, chronically ill, and immunocompromised folx. They get left behind during emergencies	1	1	en	1	t	f	\N	\N	f	f
65	2	59	67	1736720096349	1736720096349	I have not seen enough work to involved disabled folx in organizing - they are vital contributors to our movement.	1	1	en	1	t	f	\N	\N	f	f
66	2	59	67	1736721388329	1736721388329	We need to focus more efforts on checking on and helping disabled folx and the elderly.	1	1	en	1	t	f	\N	\N	f	f
67	2	59	67	1736721430195	1736721430195	We need to commit to normalizing PPE and wearing N95s - NOT just when protesting, but always. Disabled folks have warned us -we need 2listen	1	1	en	1	t	f	\N	\N	f	f
68	2	59	67	1736722158703	1736722158703	The only reason why PPE and air filters are available now is bc covid activists continued to advocate for clean air/mask since 2020.	1	1	en	0.9895866	t	f	\N	\N	f	f
69	2	61	69	1736734528766	1736734528766	I know who to organize with after the surge of mutual aid will subside after this calamity.	1	1	en	1	t	f	\N	\N	f	f
70	2	61	69	1736734572298	1736734572298	I know where to go to continue having political discussions over the future of my community.	1	1	en	1	t	f	\N	\N	f	f
72	2	0	2	1736736347307	1736736347307	Demand FEMA trailers for all unhoused, whether newly or not.	1	1	en	1	t	f	\N	\N	f	f
71	2	68	76	1736736258539	1736736258539	To inspire homeowners NOT to rebuild, the city, county, or state should buy the land so they can relocate to a safer area. Rewild!	1	1	en	1	t	f	\N	\N	f	f
73	2	76	84	1736743015736	1736743015736	During the rebuilding efforts, we should advocate for neighborhood land trusts	1	1	en	1	t	f	\N	\N	f	f
3	4	0	2	1736755015534	1736755015534	I have questions about cleaning up the toxic ash in the soil.	1	1	en	1	t	f	\N	\N	f	t
4	4	0	2	1736755039841	1736755039841	I'm worried about what contaminants might be in the soil.	1	1	en	1	t	f	\N	\N	f	t
5	4	0	2	1736756088825	1736756088825	I don't feel like I have enough information about how to stay safe from the toxic ashes	1	1	en	1	t	f	\N	\N	f	t
6	4	0	2	1736756098038	1736756098038	I want to grow food in my garden as soon as possible	1	1	en	1	t	f	\N	\N	f	t
1	4	0	2	1736754959964	1736754959964	The building I live in has burned down completely.	1	1	en	1	t	t	\N	\N	f	t
0	4	0	2	1736754941019	1736754941019	I have a home or garden that has been affected by the fires.	1	1	en	1	t	t	\N	\N	f	t
2	4	0	2	1736755000114	1736755000114	I live in an area that was within the fire perimeter.	1	1	en	1	t	t	\N	\N	f	t
4	5	0	2	1736789850838	1736789850838	I think we can focus our efforts on supporting folks who are neglected by the state, disabled, undocumented.	1	1	en	1	t	f	\N	\N	f	t
5	5	0	2	1736789904612	1736789904612	I think we should focus on political education and outreach.	1	1	en	1	t	f	\N	\N	f	t
6	5	0	2	1736791252082	1736791252082	We should organize in person de-briefs with the liberal leaning orgs and spaces who capitulated when the state shut down mutual aid operations. Educate about our political analysis.	1	1	en	1	t	f	\N	\N	f	t
0	5	0	2	1736789772241	1736789772241	I am in northeast LA.	1	-1	en	1	t	f	\N	\N	f	t
2	5	0	2	1736789794315	1736789794315	I organize in West LA and adjacent areas.	1	1	en	0.93063676	t	t	\N	\N	f	t
1	5	0	2	1736789783682	1736789783682	I organize in northeast LA.	1	1	en	0.98279434	t	t	\N	\N	f	t
3	5	0	2	1736789804761	1736789804761	I organize from a remote location.	1	1	en	1	t	t	\N	\N	f	t
7	5	0	2	1736791367922	1736791367922	We should host community care and processing events where we can do political education in the next few days. Broad outreach, attract less informed people who may be new to mutual aid.	1	1	en	1	t	f	\N	\N	f	t
8	5	0	2	1736791498674	1736791498674	We should do an coordinated multichannel media effort focused on the failures of the state and the gaps in care still present	1	1	en	1	t	f	\N	\N	f	t
12	5	0	2	1736791579142	1736791579142	I organize in SGV area.	1	1	en	0.8904795	t	t	\N	\N	f	t
11	5	0	2	1736791566251	1736791566251	I organize in SFV area.	1	1	en	1	t	t	\N	\N	f	t
10	5	0	2	1736791545632	1736791545632	I organize in South LA	1	1	en	1	t	t	\N	\N	f	t
9	5	0	2	1736791538569	1736791538569	I organize in Central LA	1	1	en	0.4379816	t	t	\N	\N	f	t
13	5	4	89	1736794490390	1736794490390	We should have canned, accessible political ed flyers/leaflets that any autonomous group can include in their distros	1	1	en	1	t	f	\N	\N	f	f
15	5	4	89	1736794601764	1736794601764	We should establish periodic phone banking to follow up with aid recipients and volunteers to bolster operations and do ongoing political ed	1	1	en	1	t	f	\N	\N	f	f
14	5	1	87	1736794575375	1736794575375	Its important to form a coherent and easily identifiable coalition org to consolidate organizers/communication and attract/educate “normies”	1	1	en	1	t	f	\N	\N	f	f
12	11	11	133	1739351728035	1739351728035	test fffff	1	0	en	0.5272742	t	f	\N	\N	f	f
16	5	5	90	1736794973358	1736794973358	I organize in Long Beach	1	1	en	1	t	t	\N	\N	f	f
17	5	8	93	1736795787951	1736795787951	we should have a coordinated response/media blast to counter the co-optation of our efforts (by the state/institutions/corporations)	1	1	en	1	t	f	\N	\N	f	f
18	5	9	94	1736795934388	1736795934388	Be mutual aid case-workers, in which we maintain consistent follow up with affected individuals to uplift each other until we’re liberated.	1	1	en	1	t	f	\N	\N	f	f
19	5	8	93	1736796202076	1736796202076	hold 'press conferences' for broadcast on trusted platforms (power leftist IG users, podcasts, etc) to put our analysis/experience out	1	0	en	1	t	f	\N	\N	f	f
20	5	8	93	1736796439123	1736796439123	organize & empower those who receive mutual aid to become a part of the organizing	1	0	en	1	t	f	\N	\N	f	f
21	5	16	42	1736807068403	1736807068403	run community health clinics	1	0	en	0.98214155	t	f	\N	\N	f	f
22	5	16	42	1736807278386	1736807278386	get some people who manage infrastructure, like sanitation, fire dept, on the same page as us	1	0	en	1	t	f	\N	\N	f	f
0	6	0	114	1737084575699	1737084575699	i am a teacher, \ni like eggs, \ni do not eat meat, \ni have at least one pet,\ni am a man,\ni have at least one sibling,\ni have seen the movie "citizen kane",\ni own a motor vehicle, \ni have been to Chuck E Cheese when I was a child, \ni have been to Chuck E Cheese as an adult,\ni have multiple siblings, \ni have been described by others as "weird"	1	1	en	1	t	f	\N	\N	f	t
1	6	0	114	1737096160063	1737096160063	i enjoy math	1	1	en	1	t	f	\N	\N	f	f
7	3	7	119	1737730363684	1737730363684	assda	1	0	uz	0.53880614	t	f	\N	\N	f	f
8	3	7	119	1737730617963	1737730617963	I think wild fires are scary	1	0	en	1	t	f	\N	\N	f	f
0	11	0	122	1739296894827	1739296894827	First comment	1	1	en	1	t	f	\N	\N	f	t
1	11	0	122	1739296903331	1739296903331	another comment	1	1	en	0.98609865	t	f	\N	\N	f	t
2	11	9	119	1739302658313	1739302658313	This is a comment from me!	1	0	en	1	t	f	\N	\N	f	f
3	11	10	132	1739308174177	1739308174177	XID comment	1	0	en	0.45821363	t	f	\N	\N	f	f
4	11	10	132	1739308313946	1739308313946	HI	1	0	en	0.30078125	t	f	\N	\N	f	f
5	11	11	133	1739308729641	1739308729641	new	1	0	en	1	t	f	\N	\N	f	f
6	11	11	133	1739308763315	1739308763315	New New	1	0	en	1	t	f	\N	\N	f	f
7	11	11	133	1739316118535	1739316118535	This is a test	1	0	en	1	t	f	\N	\N	f	f
8	11	12	135	1739316197449	1739316197449	THis is a test	1	0	en	1	t	f	\N	\N	f	f
9	11	12	135	1739316275282	1739316275282	Posting test	1	0	en	1	t	f	\N	\N	f	f
10	11	13	171	1739318781190	1739318781190	Testing if the fix worked	1	0	en	1	t	f	\N	\N	f	f
11	11	11	133	1739351006922	1739351006922	Another test	1	0	en	1	t	f	\N	\N	f	f
13	11	11	133	1739351744784	1739351744784	dsfsdf	1	0	en	1	t	f	\N	\N	f	f
1	9	1	137	1739317231797	1739317231797	Soyons vigilants à la souveraineté de la donnée francophone, et à sa propriété intellectuelle dans un monde génératif dérégulé	1	-1	fr	0.95832986	t	f	\N	\N	f	f
2	9	4	176	1739378620464	1739378620464	Il faut proposer un moratoire sur l'IA générative, tant que ses effets néfastes pour les salariés ne sont pas sous contrôle	1	-1	fr	0.97063726	t	f	\N	\N	f	f
7	9	6	177	1739381424046	1739381424046	Il y a nécessité d'envisager des droits et des protections plus étendus en ce qui concerne les données des travailleurs.	1	-1	fr	1	t	f	\N	\N	f	f
8	9	6	177	1739381437610	1739381437610	Le paradoxe, c’est que le salarié est en faveur de choisir ses usages IA générative, tout en refusant les IA imposées par l’entreprise.	1	-1	fr	0.9819505	t	f	\N	\N	f	f
3	9	6	177	1739381298561	1739381298561	Nous avons besoin d’un programme européen ambitieux de développement.	1	-1	fr	1	t	f	\N	\N	f	f
4	9	6	177	1739381326644	1739381326644	Nous avons besoin d’un programme européen ambitieux de régulation.	1	-1	fr	1	t	f	\N	\N	f	f
5	9	6	177	1739381383019	1739381383019	Augmenter l'impact IA : c’est examiner comment le travail est organisé, valorisé ; comment les industries sont structurées.	1	-1	fr	1	t	f	\N	\N	f	f
6	9	6	177	1739381406823	1739381406823	La dévaluation du travail passe par la réduction du travail des opérateurs à des données et la perte de contrôle sur leur collecte.	1	-1	fr	0.98822165	t	f	\N	\N	f	f
9	9	6	177	1739381449372	1739381449372	Les heures gagnées grâce à l’IA générative sont une fuite de productivité inutile parce qu’elles ne sont pas réutilisées.	1	-1	fr	0.98076206	t	f	\N	\N	f	f
10	9	6	177	1739381465619	1739381465619	Les incitations fiscales peuvent être un instrument de politique d’emploi face au déploiement d’IA générative.	1	-1	fr	0.9692159	t	f	\N	\N	f	f
11	9	6	177	1739381475523	1739381475523	La réglementation doit devenir un soutien : trop de réglementations contradictoires, des cadres peu clairs empêchent le passage à l'échelle.	1	-1	fr	0.98549217	t	f	\N	\N	f	f
12	9	6	177	1739381490682	1739381490682	On voit dans les exemples actuels que l’IA n’augmente pas l’emploi mais le détruit.	1	-1	fr	0.9361922	t	f	\N	\N	f	f
4	12	0	122	1739382045260	1739382045260	Augmenter l'impact IA : c’est examiner comment le travail est organisé, valorisé ; comment les industries sont structurées.	1	1	fr	1	t	f	\N	\N	f	f
13	12	0	122	1739382202220	1739382202220	L'IA fait plus que jamais l'objet de débat public. Elle doit devenir un commun démocratique.	1	1	fr	0.934886	t	f	\N	\N	f	f
1	12	0	122	1739381958818	1739381958818	Soyons vigilants à la souveraineté de la donnée francophone, et à sa propriété intellectuelle dans un monde génératif dérégulé.	1	1	fr	0.95832986	t	f	\N	\N	f	f
2	12	0	122	1739382030947	1739382030947	Nous avons besoin d’un programme européen ambitieux de développement.	1	1	fr	1	t	f	\N	\N	f	f
17	12	0	122	1739382236494	1739382236494	Après la fracture numérique, la fracture IA va être encore plus clivante dans les organisations.	1	1	fr	0.9830669	t	f	\N	\N	f	f
5	12	0	122	1739382053076	1739382053076	La dévaluation du travail passe par la réduction du travail des opérateurs à des données et la perte de contrôle sur leur collecte.	1	-1	fr	0.98822165	t	f	\N	\N	f	f
11	12	0	122	1739382106936	1739382106936	On voit dans les exemples actuels que l’Ia n’augmente pas l’emploi mais le détruit.	1	1	fr	0.9361922	t	f	\N	\N	f	f
3	12	0	122	1739382037870	1739382037870	Nous avons besoin d’un programme européen ambitieux de régulation.	1	1	fr	1	t	f	\N	\N	f	f
6	12	0	122	1739382061108	1739382061108	Il y a nécessité d'envisager des droits et des protections plus étendus en ce qui concerne les données des travailleurs.	1	1	fr	1	t	f	\N	\N	f	f
16	12	0	122	1739382229848	1739382229848	Les services IA doivent pouvoir s'adapter à tous les utilisateurs les “surchargés” comme les “disponibles.”	1	-1	fr	0.9779177	t	f	\N	\N	f	f
9	12	0	122	1739382090304	1739382090304	Les incitations fiscales peuvent être un instrument de politique d’emploi face au déploiement d’IA générative.	1	-1	fr	0.9692159	t	f	\N	\N	f	f
7	12	0	122	1739382073487	1739382073487	Le paradoxe, c’est que le salarié est en faveur de choisir ses usages IA générative, tout en refusant les IA imposées par l’entreprise.	1	1	fr	0.9819505	t	f	\N	\N	f	f
10	12	0	122	1739382097649	1739382097649	La réglementation doit devenir un soutien : trop de réglementations contradictoires, des cadres peu clairs empêchent le passage à l'échelle.	1	-1	fr	0.98549217	t	f	\N	\N	f	f
12	12	0	122	1739382186095	1739382186095	Le monde à l’envers du management qui croit les outils qui génèrent trop de faux-positifs mais pas les salariés qui doivent les corriger.	1	-1	fr	0.92724705	t	f	\N	\N	f	f
14	12	0	122	1739382213053	1739382213053	La boussole IA est une manière de prioriser des projets, si et seulement si la techno s'avère véritablement nécessaire pour le bien commun.	1	1	fr	0.9098903	t	f	\N	\N	f	f
25	12	0	122	1739382318576	1739382318576	Il faut mettre en place les conditions d'un réel dialogue social technologique au travail.	1	1	fr	1	t	f	\N	\N	f	f
19	12	0	122	1739382266494	1739382266494	80% de transformation c’est d'accompagnement l'humain et seulement 20% est de la  technologie.	1	1	fr	1	t	f	\N	\N	f	f
15	12	0	122	1739382223345	1739382223345	On doit déployer nos propres LLMs parce qu'on manipule des données usagers dont on ne peut pas se permettre de laisser fuiter des données.	1	-1	fr	1	t	f	\N	\N	f	f
21	12	0	122	1739382283570	1739382283570	Se limiter aux early adopteurs pour diffuser? Il faut avoir la bonne cadence de transformation pour chaque population dans l'organisation.	1	1	fr	1	t	f	\N	\N	f	f
26	12	0	122	1739382325339	1739382325339	Appréhender l'IA ne se limite pas à l'acquisition de compétences techniques et interroge la notion d'expertise et de management.	1	1	fr	0.9830669	t	f	\N	\N	f	f
27	12	0	122	1739382331419	1739382331419	Il faut accepter de laisser les salariés acteurs de leurs choix et de leurs usages numériques au travail.	1	1	fr	1	t	f	\N	\N	f	f
23	12	0	122	1739382299814	1739382299814	Il ne faut pas avoir peur de l'IA, mais il ne faut pas non plus avoir peur d’un dialogue social de co-construction.	1	1	fr	1	t	f	\N	\N	f	f
18	12	0	122	1739382252107	1739382252107	C'est important de sensibiliser à la sécurité, transparence ; à la façon de traiter, utiliser les données quand on n'a pas culture de data.	1	1	fr	0.96454114	t	f	\N	\N	f	f
24	12	0	122	1739382306534	1739382306534	Une IA non-biaisée n’existe pas. Mais on doit mesurer et corriger les préjugés sexistes dans l'IA pour un système qui convienne à chacun.	1	-1	fr	0.9637928	t	f	\N	\N	f	f
20	12	0	122	1739382273226	1739382273226	Sur certains thèmes, on fera le choix de ne pas lancer des projets d’IA générative.	1	1	fr	0.97894603	t	f	\N	\N	f	f
8	12	0	122	1739382082610	1739382082610	Les heures gagnées grâce à l’IA générative sont une fuite de productivité inutile parce qu’elles ne sont pas réutilisées.	1	1	fr	0.98076206	t	f	\N	\N	f	f
22	12	0	122	1739382290347	1739382290347	Nous devons penser aux générations futures qui seront touchées par nos expériences avec l’IA incontrôlées aux conséquences considérables.	1	1	fr	1	t	f	\N	\N	f	f
16	9	5	122	1739383617731	1739383617731	On doit déployer nos propres LLMs parce qu'on manipule des données usagers dont on ne peut pas se permettre de laisser fuiter des données.	1	-1	fr	1	t	f	\N	\N	f	f
14	9	5	122	1739383587020	1739383587020	L'IA fait plus que jamais l'objet de débat public. Elle doit devenir un commun démocratique.	1	-1	fr	0.934886	t	f	\N	\N	f	f
15	9	5	122	1739383607549	1739383607549	La boussole IA est une manière de prioriser des projets, si et seulement si la techno s'avère véritablement nécessaire pour le bien commun.	1	-1	fr	0.9098903	t	f	\N	\N	f	f
17	9	5	122	1739383623331	1739383623331	Les services IA doivent pouvoir s'adapter à tous les utilisateurs les “surchargés” comme les “disponibles.”	1	-1	fr	0.9779177	t	f	\N	\N	f	f
18	9	5	122	1739383629461	1739383629461	Après la fracture numérique, la fracture IA va être encore plus clivante dans les organisations.	1	-1	fr	0.9830669	t	f	\N	\N	f	f
19	9	5	122	1739383635989	1739383635989	C'est important de sensibiliser à la sécurité, transparence ; à la façon de traiter, utiliser les données quand on n'a pas culture de data.	1	-1	fr	0.96454114	t	f	\N	\N	f	f
20	9	5	122	1739383641342	1739383641342	80% de transformation c’est d'accompagnement l'humain et seulement 20% est de la  technologie.	1	-1	fr	1	t	f	\N	\N	f	f
0	12	0	122	1739381900518	1739381900518	Le temps de la régulation IA est derrière nous, désormais il faut embarquer les salariés pour qu’aucun ne reste sur le bord de la route.	1	1	fr	0.9706373	t	f	\N	\N	f	f
0	9	0	170	1739317205279	1739317205279	Le temps de la régulation IA est derrière nous, désormais il faut embarquer les salariés pour qu’aucun ne reste sur le bord de la route	1	-1	fr	0.9706373	t	f	\N	\N	f	f
13	9	5	122	1739383570911	1739383570911	Le monde à l’envers du management qui croit les outils qui génèrent trop de faux-positifs mais pas les salariés qui doivent les corriger.	1	-1	fr	0.92724705	t	f	\N	\N	f	f
21	9	5	122	1739383648000	1739383648000	Sur certains thèmes, on fera le choix de ne pas lancer des projets d’IA générative.	1	-1	fr	0.97894603	t	f	\N	\N	f	f
26	9	5	122	1739383680893	1739383680893	Il faut mettre en place les conditions d'un réel dialogue social technologique au travail.	1	-1	fr	1	t	f	\N	\N	f	f
28	9	5	122	1739383694901	1739383694901	Il faut accepter de laisser les salariés acteurs de leurs choix et de leurs usages numériques au travail.	1	-1	fr	1	t	f	\N	\N	f	f
22	9	5	122	1739383655586	1739383655586	Se limiter aux early adopteurs pour diffuser? Il faut avoir la bonne cadence de transformation pour chaque population dans l'organisation.	1	-1	fr	1	t	f	\N	\N	f	f
25	9	5	122	1739383674001	1739383674001	Une IA non-biaisée n’existe pas. Mais on doit mesurer et corriger les préjugés sexistes dans l'IA pour un système qui convienne à chacun.	1	-1	fr	0.9637928	t	f	\N	\N	f	f
27	9	5	122	1739383688686	1739383688686	Appréhender l'IA ne se limite pas à l'acquisition de compétences techniques et interroge la notion d'expertise et de management.	1	-1	fr	0.9830669	t	f	\N	\N	f	f
23	9	5	122	1739383662549	1739383662549	Nous devons penser aux générations futures qui seront touchées par nos expériences avec l’IA incontrôlées aux conséquences considérables.	1	-1	fr	1	t	f	\N	\N	f	f
24	9	5	122	1739383668330	1739383668330	Il ne faut pas avoir peur de l'IA, mais il ne faut pas non plus avoir peur d’un dialogue social de co-construction.	1	-1	fr	1	t	f	\N	\N	f	f
29	9	8	179	1739453811399	1739453811399	La ministre du travail qui parle de dialogue social, alors que les gouvernements récents ont plutôt laminé ces structures. Drole.	1	0	fr	0.9841998	t	f	\N	\N	f	f
30	9	9	180	1739454039928	1739454039928	Après le test ?	1	0	fr	1	t	f	\N	\N	f	f
31	9	8	179	1739454685535	1739454685535	Si impact de l'IA est intensification et développement des inégalités, alors comment on régule les cadences ?	1	0	fr	0.9680646	t	f	\N	\N	f	f
28	12	0	122	1739455244995	1739455244995	Ceux qui s'en servent en secret ne devraient pas le dire pour continuer à profiter du gain pour eux...	1	1	fr	0.9812663	t	f	\N	\N	f	t
29	12	0	122	1739455658441	1739455658441	L'usage de l'IA est un sautien individuel fort pour l'utiliseur individuel, mais un apport décevant pour le collectif	1	1	fr	0.9438174	t	f	\N	\N	f	t
30	12	0	122	1739456061549	1739456061549	La plupart du temps, un savoir collectif passé est abusivement utilisé par l'ia pour donner des ordres aux salariés et supprimer leur manager	1	1	fr	0.9745296	t	f	\N	\N	f	t
32	9	11	209	1739456075234	1739456075234	Pour protéger l’emploi et les travailleurs, le périmètre d’action des agents IA doit être limité à certaines tâches.	1	0	fr	0.9576763	t	f	\N	\N	f	f
33	9	11	209	1739456366335	1739456366335	Comme le nutri-score qui évalue la qualité des aliments que nous ingérons, mettre en place un indicateur pour les niveaux intrusifs des IA	1	0	fr	0.936515	t	f	\N	\N	f	f
34	9	11	209	1739456573839	1739456573839	Pour sensibiliser en masse (dont les travailleurs) à l’IA et à ses enjeux, initier une émission sur les chaînes télévisées nationales.	1	0	fr	0.9593978	t	f	\N	\N	f	f
35	9	11	209	1739456650416	1739456650416	Dans la continuité de ma proposition d’émission télévisée nationale,former aux fondamentaux et accompagner nos citoyens à sa prise en main.	1	0	fr	0.9796053	t	f	\N	\N	f	f
36	9	11	209	1739456937270	1739456937270	Dans chaque entreprise, mettre en place cellule « Notre entreprise plus compétitive demain » Besoin de plus de caractères pour expliquer :)	1	0	fr	1	t	f	\N	\N	f	f
37	9	8	179	1739457013459	1739457013459	Oui, faire que les gains de productivité grâce à l'IA soient redistribués aux salariés.	1	0	fr	0.94850415	t	f	\N	\N	f	f
38	9	8	179	1739457711313	1739457711313	Proposer que les entreprises qui recourent à l'iA s'engagent à ne pas débaucher dans le temps de déploiement + X annees	1	0	fr	0.96472585	t	f	\N	\N	f	f
31	12	0	122	1739457730941	1739457730941	Pour que l'introduction IA ne soit pas une punition, chaque salarié doit avoir son plan de formation et une compréhension de son intérêt	1	1	fr	1	t	f	\N	\N	f	t
32	12	0	122	1739457992775	1739457992775	Former des experts, c'est commencer par les activités basiques puis en faire de plus complexes. Avec l'IA, ça disparaît ?	1	1	fr	1	t	f	\N	\N	f	t
33	12	0	122	1739458495014	1739458495014	Les IA devraient passer un test de conformité produit avant introduction sur le marché. Comme tout produit.	1	1	fr	0.98975265	t	f	\N	\N	f	t
34	12	0	122	1739458745044	1739458745044	Les agents IA ne doivent pas opérer au-delà de tâches déimitées afin de protéger les travailleurs	1	1	fr	1	t	f	\N	\N	f	t
35	12	0	122	1739463187085	1739463187085	Lorsqu'on parle d'IA générative, on évoque souvent la productivité, mais pas assez l'humain.	1	1	fr	0.96721685	t	f	\N	\N	f	t
36	12	0	122	1739464053267	1739464053267	L'IA ne nous remplace pas mais elle nous REPLACE.	1	1	fr	1	t	f	\N	\N	f	t
37	12	0	122	1739464787541	1739464787541	Il faut que nous ayons le choix de nous "opt out".	1	1	fr	1	t	f	\N	\N	f	t
38	12	0	122	1739465889785	1739465889785	Plus nous avons d'IA, plus nous avons besoin d'intelligence émotionnelle.	1	1	fr	0.9857226	t	f	\N	\N	f	t
39	12	0	122	1739466009664	1739466009664	La technologie fera ce que nous lui demanderons de faire. Nous pouvons empêcher que ce soit l'inverse.	1	1	fr	0.98796684	t	f	\N	\N	f	t
40	12	0	122	1739468072666	1739468072666	Il faut des syndicats plus forts et mieux formés.	1	1	fr	1	t	f	\N	\N	f	t
41	12	0	122	1739468200944	1739468200944	rêvons que les salariés retrouvent la défense de leurs métiers grâce à l'IA.	1	1	fr	0.94551563	t	f	\N	\N	f	t
42	12	60	245	1739596117696	1739596117696	on parle d'intelligence artificielle, je ne vous sue des pseudo intelligences superficielles	1	0	fr	1	t	f	\N	\N	f	f
43	12	60	245	1739596156218	1739596156218	lIA ne remplace pas...encore	1	0	fr	1	t	f	\N	\N	f	f
44	12	60	245	1739596375562	1739596375562	si une IA est capable de faire ton boulot, tu ne sers plus à rien	1	0	fr	0.98602426	t	f	\N	\N	f	f
45	12	60	245	1739596514332	1739596514332	il est nécessaire de revoir les approches de formation des humains car si une IA peut faire les exposés ou est l'apprentissage (des humains)	1	0	fr	1	t	f	\N	\N	f	f
46	12	60	245	1739596748902	1739596748902	etonnament les discours en faveur de l'IA apparaissent déconnectés des préoccupations environnementale.	1	0	fr	0.9716986	t	f	\N	\N	f	f
\.


--
-- Data for Name: contexts; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.contexts (context_id, name, creator, is_public, created) FROM stdin;
\.


--
-- Data for Name: contributer_agreement_signatures; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.contributer_agreement_signatures (uid, name, company_name, github_id, email, agreement_version, created) FROM stdin;
\.


--
-- Data for Name: conversation_translations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.conversation_translations (zid, src, topic, description, lang, created, modified) FROM stdin;
\.


--
-- Data for Name: conversations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.conversations (zid, topic, description, link_url, parent_url, upvotes, participant_count, is_anon, is_active, is_draft, is_public, is_data_open, profanity_filter, spam_filter, strict_moderation, prioritize_seed, vis_type, write_type, help_type, write_hint_type, style_btn, socialbtn_type, subscribe_type, branding_type, bgcolor, help_bgcolor, help_color, email_domain, use_xid_whitelist, owner, org_id, context, course_id, owner_sees_participation_stats, auth_needed_to_vote, auth_needed_to_write, auth_opt_fb, auth_opt_tw, auth_opt_allow_3rdparty, modified, created, importance_enabled) FROM stdin;
7			\N	\N	1	0	f	t	t	t	f	t	t	f	f	0	1	1	1	\N	0	1	1	\N	\N	\N	\N	f	1	1	\N	\N	f	f	f	f	f	t	1737721920961	1737721918134	f
1			\N	\N	1	0	f	t	t	t	f	t	t	f	f	0	1	1	1	\N	0	1	1	\N	\N	\N	\N	f	1	1	\N	\N	f	f	f	f	f	t	1736636202681	1736635873497	f
4	Post-Wildfire Soil Remediation	This is a conversation to see what issues you are concerned with around soil health, toxic ash, and post-wildfire cleanup. Please vote on more than 7 of the cards below. If you don't feel you entirely agree or disagree with a statement, please use that insight to write a statement of your own. A statement could be about any concerns you have about, your experiences or understanding of the issues, things you've noticed and other thoughts and opinions you have. Feel free to add statements about anything that you feel isn't covered by other statements in the deck.\n\n\n\n\nAll statements of opinions, ideas, and experiences are welcome. Please don't share any identifying information - Doxxing or sharing of private info is not tolerated and will be removed.\n\n\n	\N	\N	1	2	f	t	t	t	f	t	t	f	f	1	1	1	1	\N	0	0	1	\N	\N	\N	\N	f	2	2	\N	\N	f	f	f	f	f	t	1736756116183	1736754529901	f
2	2025 Los Angeles Wildfires Response	The wildfires have highlighted many issues in the Los Angeles area. This is a conversation to see where and how we mutual aid participants/organizers agree upon issues we'd like to address, and actions we can take to do so.\n\nFeel free to add your own statements about any issues you experience or notice.\n\nIf you don't fully agree or disagree with a statement, use the clarity from that insight to write your own statement.\n\n\nAll statements of opinions, ideas, and experiences are welcome. Doxxing or sharing of private info is not tolerated and will be removed.\n\n\n	\N	\N	1	93	f	t	t	t	f	t	t	f	f	1	1	1	1	\N	0	0	1	\N	\N	\N	\N	f	2	2	\N	\N	f	f	f	f	f	t	1737467674727	1736644696153	f
6	Test topic please ignore	This is a test conversation created by Mr Lucas. This is NOT an official conversation, and is meant for participants to further understand the features and potential usefulness of the poliscommunity ap	\N	\N	1	1	f	t	t	t	f	t	t	f	f	1	1	1	1	\N	0	1	1	\N	\N	\N	\N	f	114	114	\N	\N	f	f	f	f	f	t	1737096160075	1736958249631	f
9	Paris AI (test)		\N	\N	1	15	f	t	t	t	f	t	t	f	f	0	1	0	1	\N	0	0	1	\N	\N	\N	\N	f	122	122	\N	\N	f	f	f	f	f	t	1739462776620	1739182649063	f
5	Regrouping mutual aid fire Responses	In light of the all the hubs and autonomous shelters being closed by the city and Red Cross - we'd like to regroup our efforts. How do you think we should focus? What concerns do you have?	\N	\N	1	21	f	t	t	t	f	t	t	f	f	1	1	1	1	\N	0	0	1	\N	\N	\N	\N	f	2	2	\N	\N	f	f	f	f	f	t	1736985982519	1736787182231	f
12	Paris AI Summit		\N	\N	1	61	f	t	t	t	f	t	t	t	f	1	1	0	1	\N	0	0	1	\N	\N	\N	\N	f	122	122	\N	\N	f	f	f	f	f	t	1739596748921	1739381669815	f
3	test	Stuarts test convo	\N	\N	1	8	f	t	t	t	f	t	t	f	f	0	1	0	1	\N	0	0	1	\N	\N	\N	\N	f	1	1	\N	\N	f	f	f	f	f	t	1737731284723	1736648068886	f
8			\N	\N	1	0	f	t	t	t	f	t	t	f	f	0	1	1	1	\N	0	1	1	\N	\N	\N	\N	f	120	120	\N	\N	f	f	f	f	f	t	1737739910176	1737739910176	f
13			\N	\N	1	0	f	t	t	t	f	t	t	f	f	0	1	1	1	\N	0	1	1	\N	\N	\N	\N	f	244	244	\N	\N	f	f	f	f	f	t	1739539462327	1739539462327	f
10			\N	\N	1	0	f	t	t	t	f	t	t	f	f	0	1	1	1	\N	0	1	1	\N	\N	\N	\N	f	123	123	\N	\N	f	f	f	f	f	t	1739197860101	1739197860101	f
11	test convo	This is a test convo	\N	\N	1	16	f	t	t	t	f	t	t	f	f	0	1	1	1	\N	0	1	1	\N	\N	\N	\N	f	122	122	\N	\N	f	f	f	f	f	t	1739363141590	1739296865027	f
\.


--
-- Data for Name: courses; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.courses (course_id, topic, description, owner, course_invite, created) FROM stdin;
\.


--
-- Data for Name: crowd_mod; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.crowd_mod (zid, pid, tid, created, as_important, as_factual, as_feeling, as_notmyfeeling, as_notgoodidea, as_notfact, as_unsure, as_spam, as_abusive, as_offtopic) FROM stdin;
\.


--
-- Data for Name: demographic_data; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.demographic_data (uid, fb_gender, ms_birth_year_estimate_fb, ms_gender_estimate_fb, fb_timestamp, ms_fb_timestamp, ms_response, gender_guess, birth_year_guess) FROM stdin;
\.


--
-- Data for Name: einvites; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.einvites (einvite, email, created) FROM stdin;
\.


--
-- Data for Name: email_validations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.email_validations (email, created) FROM stdin;
\.


--
-- Data for Name: event_ptpt_no_more_comments; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.event_ptpt_no_more_comments (zid, pid, votes_placed, created) FROM stdin;
3	1	2	1736648231405
3	2	3	1736651757195
3	3	4	1736651774023
3	4	5	1736651792965
3	5	6	1736651811213
3	6	7	1736651840741
2	4	18	1736653718732
2	3	18	1736653726353
2	5	18	1736654473769
2	7	18	1736654621581
2	8	18	1736654686221
2	9	18	1736654917500
2	10	18	1736654982619
2	11	19	1736655097915
2	10	20	1736655204723
2	13	20	1736655339366
2	12	20	1736655354222
2	12	22	1736655464288
2	13	23	1736655506657
2	14	24	1736655671957
2	0	32	1736655968033
2	15	24	1736656445844
2	16	24	1736656471582
2	13	25	1736656540561
2	13	26	1736656674282
2	13	27	1736656886662
2	17	27	1736656978220
2	17	28	1736657085523
2	17	29	1736657130008
2	13	29	1736657246523
2	20	29	1736657337537
2	23	30	1736657396430
2	24	30	1736657428321
2	0	39	1736657504413
2	25	31	1736657707656
2	28	31	1736657950797
2	27	31	1736658029918
2	29	33	1736658190490
2	30	33	1736658275022
2	13	37	1736660019549
2	31	39	1736663487792
2	32	42	1736664737468
2	33	42	1736666211727
2	0	51	1736666784340
2	11	47	1736676792755
2	36	47	1736678370087
2	11	48	1736699155191
2	13	48	1736699429244
2	15	48	1736699969998
2	43	48	1736700054956
2	45	48	1736701128710
2	49	48	1736701772943
2	47	50	1736701918674
2	51	50	1736702118297
2	13	50	1736702205873
2	53	52	1736702799892
2	52	52	1736703255909
2	56	53	1736708627653
2	57	55	1736709173138
2	16	57	1736720962203
2	59	57	1736721356465
2	16	60	1736724744934
2	60	60	1736732883733
2	13	60	1736734051077
2	61	60	1736734481618
2	64	63	1736734841864
2	66	62	1736735694156
2	69	62	1736735977494
2	68	62	1736736082073
2	0	72	1736736364821
2	72	64	1736738738313
2	61	64	1736738757210
2	75	64	1736740538781
2	76	64	1736742974375
4	1	7	1736756116190
5	1	12	1736794414070
5	3	13	1736794542531
5	1	14	1736794584849
5	1	15	1736794607638
5	0	16	1736794796038
5	5	15	1736794963067
5	7	16	1736795629478
5	8	16	1736795633296
5	9	18	1736795963224
5	8	18	1736796013811
5	9	19	1736796245506
5	9	20	1736797017866
5	10	20	1736797186813
5	11	20	1736797908543
5	13	20	1736802941647
5	14	20	1736803951286
5	9	22	1736808503747
5	17	22	1736809832986
5	8	22	1736809839098
5	18	22	1736814202580
5	20	22	1736814656700
2	78	65	1736827957705
2	11	65	1736879545649
2	82	65	1736881039906
2	85	65	1736881928831
2	86	65	1736882632970
2	87	65	1736917361966
5	12	22	1736985982528
2	90	65	1737041950040
2	13	65	1737062909855
2	91	65	1737265536041
2	92	65	1737467674757
11	1	2	1739296923131
11	2	2	1739296934180
11	3	2	1739296943390
11	4	2	1739296950859
11	5	2	1739296960304
11	6	2	1739296970148
11	8	2	1739296990935
9	2	2	1739345831854
9	3	2	1739371792565
9	4	3	1739378716688
9	4	4	1739378807994
9	5	19	1739384339241
9	7	1	1739384417654
9	9	1	1739453994113
12	2	2	1739454349406
9	10	2	1739454378245
12	3	2	1739454402021
12	5	3	1739454500401
12	4	3	1739454503034
9	8	2	1739454530136
12	6	3	1739454576010
12	2	3	1739454624574
12	8	3	1739454626346
12	7	3	1739454627139
12	9	3	1739454641044
12	3	3	1739454651917
12	12	3	1739454672260
12	10	3	1739454683356
12	11	3	1739454689172
12	15	3	1739454699541
12	13	3	1739454701530
12	17	3	1739454708602
12	16	3	1739454724514
12	18	3	1739454731697
12	18	4	1739454731779
12	21	3	1739454735238
12	20	3	1739454738081
12	23	3	1739454751878
12	24	3	1739454762137
12	22	3	1739454787329
12	19	3	1739454789134
12	14	3	1739454843942
12	25	3	1739454848381
12	26	3	1739454865171
12	27	3	1739454902377
12	3	4	1739454982293
12	10	4	1739454990502
12	28	5	1739455035136
12	10	5	1739455036644
12	2	6	1739455135938
12	5	6	1739455154535
12	5	7	1739455304652
12	3	7	1739455310264
12	4	7	1739455315173
12	9	7	1739455374047
9	11	3	1739455397466
12	20	7	1739455540036
12	2	7	1739455574905
12	11	7	1739455577072
12	22	7	1739455598377
12	29	7	1739455641006
12	11	8	1739455676736
12	4	9	1739455725182
12	2	9	1739455743300
12	11	9	1739455765720
12	30	9	1739455776777
12	8	9	1739455816154
12	10	9	1739455828902
12	20	9	1739455842153
12	21	10	1739455925716
12	11	10	1739455959419
12	4	10	1739456037221
12	2	11	1739456081606
12	11	11	1739456097509
12	31	11	1739456206589
12	9	12	1739456245566
12	2	12	1739456284290
12	11	12	1739456382385
12	4	12	1739456444601
12	20	12	1739456455891
12	12	12	1739456458879
12	5	12	1739456469616
12	22	12	1739456516841
12	32	12	1739456583681
12	30	12	1739456736761
12	3	12	1739456765248
9	8	7	1739456812440
9	8	8	1739456947980
9	11	9	1739457076878
12	2	13	1739457523524
12	4	13	1739457545663
12	11	13	1739457546321
12	18	14	1739457633147
12	10	13	1739457637219
12	9	13	1739457647585
12	5	13	1739457726373
12	5	14	1739457754483
12	2	14	1739457763911
12	18	15	1739457766569
12	11	14	1739457771256
9	12	10	1739457840158
12	33	14	1739457894638
12	30	14	1739457907564
12	5	15	1739458081725
12	12	15	1739458104701
12	11	15	1739458152251
12	9	15	1739458159067
12	22	15	1739458159212
12	3	16	1739458170231
12	4	15	1739458352430
12	2	18	1739458623318
12	18	19	1739458642216
12	34	18	1739458671163
12	5	18	1739458681534
12	4	18	1739458682826
12	12	18	1739458707989
12	30	18	1739458737207
12	4	19	1739458790148
12	2	19	1739458796752
12	34	19	1739458836912
12	5	19	1739458889290
12	9	19	1739459087861
12	18	20	1739459235144
12	34	21	1739459426547
12	18	22	1739459450031
12	5	21	1739459469888
12	21	21	1739459493961
12	2	21	1739459578439
12	4	21	1739459597941
12	22	21	1739459815228
12	20	21	1739459829462
12	36	21	1739459830413
12	10	21	1739459847187
12	30	21	1739459911804
12	7	21	1739459916543
12	43	21	1739459976470
12	41	21	1739459999330
12	14	21	1739459999528
12	17	21	1739460004830
12	16	21	1739460030730
12	40	21	1739460039492
12	27	21	1739460121415
12	44	21	1739460280492
12	8	21	1739460281250
12	45	21	1739461238968
12	12	21	1739461844013
12	3	23	1739461873255
12	49	21	1739461986903
12	51	21	1739462387589
12	16	23	1739462551976
12	2	23	1739462578030
9	14	10	1739462776634
12	34	23	1739462821767
12	36	23	1739462829986
12	10	23	1739462878878
12	41	23	1739462923400
12	14	23	1739462934760
12	31	23	1739462985544
12	12	23	1739463046140
12	54	23	1739463061856
12	31	24	1739463200678
12	2	24	1739463202247
12	5	24	1739463255125
12	16	24	1739463342814
12	18	25	1739463342942
12	5	25	1739463410010
12	3	28	1739463412835
12	17	25	1739463435175
12	34	25	1739463496510
12	11	25	1739463538068
12	2	25	1739463598374
12	2	26	1739463643098
12	18	28	1739463784188
12	5	26	1739463844525
12	12	26	1739463865935
12	18	29	1739464094717
12	17	27	1739464099517
12	41	27	1739464117332
12	36	27	1739464127878
12	49	27	1739464128602
12	12	27	1739464137140
12	21	27	1739464154794
12	39	27	1739464188679
12	30	27	1739464192012
12	10	27	1739464228980
12	20	27	1739464242061
12	5	27	1739464254441
12	40	27	1739464254886
12	31	27	1739464282676
12	2	27	1739464297572
12	35	27	1739464303894
12	15	27	1739464320707
12	29	27	1739464333326
12	56	27	1739464353250
12	11	27	1739464385215
12	34	27	1739464429351
12	16	27	1739464708807
12	16	28	1739464811643
12	34	28	1739464835503
12	21	29	1739464867475
12	17	29	1739464871303
12	34	29	1739464872109
12	5	30	1739464916512
12	16	30	1739464924623
12	11	30	1739464930978
12	2	30	1739464942290
12	34	30	1739464997867
12	34	31	1739465014726
12	35	30	1739465093936
12	27	30	1739465099281
12	12	30	1739465105948
12	20	30	1739465107083
12	30	30	1739465156200
12	29	30	1739465232777
12	3	34	1739465240329
12	53	30	1739465245786
12	8	31	1739465465766
12	2	31	1739465529889
12	21	31	1739465540419
12	18	33	1739465571311
12	34	32	1739465646370
12	12	31	1739465718413
12	5	32	1739465952115
12	15	32	1739465956048
12	41	32	1739465957264
12	49	32	1739465967089
12	16	32	1739465969836
12	22	32	1739465972082
12	15	33	1739466030044
12	2	33	1739466072962
12	16	33	1739466081007
12	5	33	1739466161344
12	34	34	1739466254341
12	30	33	1739466289941
12	31	33	1739466301208
12	27	33	1739466338153
12	35	33	1739466347076
12	17	33	1739466383264
12	11	33	1739466522763
12	10	33	1739466548856
12	21	33	1739467345799
12	3	38	1739467890164
12	5	34	1739468106972
12	17	34	1739468134581
12	3	39	1739468189557
12	5	35	1739468239947
12	2	35	1739468310621
12	3	40	1739468503144
12	18	38	1739480200600
12	18	39	1739480968706
12	60	37	1739596338728
\.


--
-- Data for Name: facebook_friends; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.facebook_friends (uid, friend) FROM stdin;
\.


--
-- Data for Name: facebook_users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.facebook_users (uid, fb_user_id, fb_name, fb_link, fb_public_profile, fb_login_status, fb_auth_response, fb_access_token, fb_granted_scopes, fb_location_id, location, response, fb_friends_response, created, modified) FROM stdin;
\.


--
-- Data for Name: inviters; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.inviters (inviter_uid, invited_email, created) FROM stdin;
\.


--
-- Data for Name: jianiuevyew; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.jianiuevyew (uid, pwhash) FROM stdin;
1	$2a$12$scwUCYk9Bze2NRTxOHfSEe2fAnP84OZ5wpSUeNeb7zfzgTgawYP8W
2	$2a$12$aXp8KL80yk3tcBsRFuiXG.TTWDaGksAeB40wLNFNG91uoGy9yueLO
114	$2a$12$twk0WnTV4x66RjGcBawTj.R5Zrrie37UwxJGsyyITQoqCv3aT8bbS
120	$2a$12$W4H2ikxPl3gGZRrM95JHLORl1nnmzqvbLPWKzY1GnaqKzMGvy8iim
121	$2a$12$YkISemC2nmIMmUNGx7hp7OkHj2M3.1LRlfcFrpo2OHG8jTtvjwfVy
122	$2a$12$MPmH7iq8.lviNy53.28zKOP/E9wVfPXu1cm0WNbgwYT2n7phFhiAq
123	$2a$12$2F472Aka8mElWm3vT44b1OKrxgeYzyNJppLLJOkWJ8PPXp6E0vX/a
244	$2a$12$exGQAs2sNDyXXeWovtqBAeQNYpQ4gG.Cq5iHLb3eDmJNO98qbP1cy
246	$2a$12$K/1Y/fnGOrFxd0GVL9aQ.OgJh03bsUR4k5ZiDBIR3A6r1RNbgb4Uu
\.


--
-- Data for Name: math_bidtopid; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.math_bidtopid (zid, math_env, math_tick, data, modified) FROM stdin;
12	prod	1259	{"zid": 12, "bidToPid": [[29], [4], [6], [0], [30], [21], [26], [9], [32], [20], [2], [5], [22], [3], [11], [33], [12], [13], [36], [18], [10], [7], [38], [17], [14], [27], [40], [16], [8], [43], [44], [45], [41], [35], [31], [34], [1], [39], [49], [51], [52], [53], [54], [56], [15], [57], [59], [60]], "lastVoteTimestamp": 1739596748921}	1739777951244
4	prod	259	{"zid": 4, "bidToPid": [[0], [1]], "lastVoteTimestamp": 1736756116183}	1737610828828
5	prod	616	{"zid": 5, "bidToPid": [[0], [3], [5], [6], [7], [9], [12], [13], [4], [14], [15], [10, 11], [16], [1], [17], [8], [18], [20], [2]], "lastVoteTimestamp": 1736985982519}	1737841229529
6	prod	223	{"zid": 6, "bidToPid": [[0]], "lastVoteTimestamp": 1737096160075}	1737956428661
3	prod	485	{"zid": 3, "bidToPid": [[1], [0], [2], [3], [4], [5], [6], [7]], "lastVoteTimestamp": 1737731284723}	1738590028730
11	prod	185	{"zid": 11, "bidToPid": [[1], [2], [3], [4], [5], [9, 11, 12, 13, 14], [10], [0, 6, 8], [7]], "lastVoteTimestamp": 1739363141590}	1739777950688
9	prod	259	{"zid": 9, "bidToPid": [[0, 2, 3, 4, 5, 1, 7, 6], [9], [8], [10], [11], [13], [12, 14]], "lastVoteTimestamp": 1739462776620}	1739777950582
2	prod	3438	{"zid": 2, "bidToPid": [[2], [3], [1], [0], [4], [5], [6], [7], [8], [9], [10], [11], [12], [13], [14], [15], [16], [17], [18], [19], [21], [20], [22], [23], [24], [25], [26], [27], [28], [29], [30], [31], [32], [33], [36], [38], [39], [41], [43], [44], [45], [47], [49], [50], [51], [52], [53], [55], [56], [57], [58], [59], [60], [61], [63], [64], [65], [62], [66], [67], [68], [69], [71], [72], [73], [70], [74], [75], [76], [77], [78], [79], [81], [82], [84], [85], [86], [87], [88], [89], [90], [91], [92]], "lastVoteTimestamp": 1737467674727}	1738330829692
\.


--
-- Data for Name: math_cache; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.math_cache (zid, math_env, data, modified) FROM stdin;
\.


--
-- Data for Name: math_exportstatus; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.math_exportstatus (zid, math_env, filename, data, modified) FROM stdin;
\.


--
-- Data for Name: math_main; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.math_main (zid, math_env, data, last_vote_timestamp, caching_tick, math_tick, modified) FROM stdin;
6	prod	{"n": 1, "pca": {"comps": [[0.0, 0.0]], "center": [0.0, -1.0], "comment-extremity": [0.0, 0.0], "comment-projection": [[0.0, 0.0], [0.0, 0.0]]}, "zid": 6, "tids": [0, 1], "mod-in": [0, 1], "n-cmts": 2, "in-conv": [0], "mod-out": [], "repness": {"0": [{"tid": 1, "p-test": 1.4142135623730951, "n-agree": 1, "repness": 1.333333333333333, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "best-agree": true, "repful-for": "agree", "repness-test": 0.0}]}, "consensus": {"agree": [{"tid": 1, "p-test": 1.4142135623730951, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667}], "disagree": []}, "meta-tids": [], "votes-base": {"0": {"A": [0], "D": [0], "S": [1]}, "1": {"A": [1], "D": [0], "S": [1]}}, "group-votes": {"0": {"votes": {"0": {"A": 0, "D": 0, "S": 1}, "1": {"A": 1, "D": 0, "S": 1}}, "n-members": 1}}, "base-clusters": {"x": [0.0], "y": [0.0], "id": [0], "count": [1], "members": [[0]]}, "group-clusters": [{"id": 0, "center": [0.0, 0.0], "members": [0]}], "subgroup-votes": {"0": {"0": {"votes": {"0": {"A": 0, "D": 0, "S": 1}, "1": {"A": 1, "D": 0, "S": 1}}, "n-members": 1}}}, "lastModTimestamp": null, "subgroup-repness": {"0": {"0": [{"tid": 1, "p-test": 1.4142135623730951, "n-agree": 1, "repness": 1.333333333333333, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "best-agree": true, "repful-for": "agree", "repness-test": 0.0}]}}, "user-vote-counts": {"0": 2}, "lastVoteTimestamp": 1737096160075, "subgroup-clusters": {"0": [{"id": 0, "center": [0.0, 0.0], "members": [0], "parent-id": 0}]}, "comment-priorities": {"0": 0.7831078906305026, "1": 12.529726250088048}, "group-aware-consensus": {"0": 0.3333333333333333, "1": 0.6666666666666666}}	1737096160075	4644	223	1737956428641
11	prod	{"n": 16, "pca": {"comps": [[0.7071073345726294, -0.7071062278000326, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], [-0.7071062278000325, -0.7071073345726293, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]], "center": [0.11111111111111113, 0.11111111111111113, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0], "comment-extremity": [4.15739709641549, 4.15739709641549, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], "comment-projection": [[-2.9397259796063464, 2.939721378313166, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], [2.9397213783131657, 2.939725979606346, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]]}, "zid": 11, "tids": [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13], "mod-in": [0, 1], "n-cmts": 14, "in-conv": [0, 7, 1, 4, 13, 6, 3, 12, 2, 11, 9, 5, 14, 10, 8], "mod-out": [], "repness": {"0": [{"tid": 10, "p-test": 1.0, "repness": 2, "n-trials": 0, "n-success": 0, "p-success": 0.5, "repful-for": "disagree", "repness-test": 1.1547005}], "1": [{"tid": 2, "p-test": 1.7320508075688772, "n-agree": 2, "repness": 1.5, "n-trials": 2, "n-success": 2, "p-success": 0.75, "best-agree": true, "repful-for": "agree", "repness-test": 0.0}, {"tid": 1, "p-test": 1.7320508075688772, "repness": 3.375, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "disagree", "repness-test": 2.2248595}]}, "consensus": {"agree": [{"tid": 2, "p-test": 1.7320508075688772, "n-trials": 2, "n-success": 2, "p-success": 0.75}, {"tid": 4, "p-test": 1.7320508075688772, "n-trials": 2, "n-success": 2, "p-success": 0.75}, {"tid": 5, "p-test": 1.7320508075688772, "n-trials": 2, "n-success": 2, "p-success": 0.75}, {"tid": 10, "p-test": 1.7320508075688772, "n-trials": 2, "n-success": 2, "p-success": 0.75}, {"tid": 11, "p-test": 1.7320508075688772, "n-trials": 2, "n-success": 2, "p-success": 0.75}], "disagree": []}, "meta-tids": [], "votes-base": {"0": {"A": [1, 0, 1, 0, 0, 0, 0, 0, 0], "D": [0, 1, 0, 1, 0, 0, 1, 0, 0], "S": [1, 1, 1, 1, 1, 0, 1, 3, 0]}, "1": {"A": [1, 0, 0, 1, 0, 0, 0, 0, 0], "D": [0, 1, 1, 0, 1, 0, 0, 0, 0], "S": [1, 1, 1, 1, 1, 0, 0, 3, 1]}, "2": {"A": [0, 0, 0, 0, 0, 1, 1, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 1, 1, 0, 0]}, "3": {"A": [0, 0, 0, 0, 0, 0, 1, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 0, 1, 0, 0]}, "4": {"A": [0, 0, 0, 0, 0, 1, 1, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 1, 1, 0, 0]}, "5": {"A": [0, 0, 0, 0, 0, 2, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 2, 0, 0, 0]}, "6": {"A": [0, 0, 0, 0, 0, 1, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 1, 0, 0, 0]}, "7": {"A": [0, 0, 0, 0, 0, 1, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 1, 0, 0, 0]}, "8": {"A": [0, 0, 0, 0, 0, 1, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 1, 0, 0, 0]}, "9": {"A": [0, 0, 0, 0, 0, 1, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 1, 0, 0, 0]}, "10": {"A": [0, 0, 0, 0, 0, 2, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 2, 0, 0, 0]}, "11": {"A": [0, 0, 0, 0, 0, 1, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 1, 0, 0, 0]}, "12": {"A": [0, 0, 0, 0, 0, 1, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 1, 0, 0, 0]}, "13": {"A": [0, 0, 0, 0, 0, 1, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 1, 0, 0, 0]}}, "group-votes": {"0": {"votes": {"0": {"A": 2, "D": 1, "S": 6}, "1": {"A": 2, "D": 1, "S": 7}, "2": {"A": 0, "D": 0, "S": 0}, "3": {"A": 0, "D": 0, "S": 0}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 0, "D": 0, "S": 0}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 0, "D": 0, "S": 0}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 0, "D": 0, "S": 0}}, "n-members": 7}, "1": {"votes": {"0": {"A": 0, "D": 2, "S": 3}, "1": {"A": 0, "D": 2, "S": 2}, "2": {"A": 2, "D": 0, "S": 2}, "3": {"A": 1, "D": 0, "S": 1}, "4": {"A": 2, "D": 0, "S": 2}, "5": {"A": 2, "D": 0, "S": 2}, "6": {"A": 1, "D": 0, "S": 1}, "7": {"A": 1, "D": 0, "S": 1}, "8": {"A": 1, "D": 0, "S": 1}, "9": {"A": 1, "D": 0, "S": 1}, "10": {"A": 2, "D": 0, "S": 2}, "11": {"A": 1, "D": 0, "S": 1}, "12": {"A": 1, "D": 0, "S": 1}, "13": {"A": 1, "D": 0, "S": 1}}, "n-members": 8}}, "base-clusters": {"x": [-0.0000032536056099513395, 0.0000026028844880785667, -3.7416577121333567, 3.741657061412235, -1.8708275546244344, 0.0, 1.1758903918425385, -0.00000032536056100982084, 0.29397213783131665], "y": [4.1573970964142175, -3.3259176771313737, 0.41573678139637277, 0.4157426378864708, -1.4550904478675004, 0.0, -1.1758885513252662, 0.41573970964142176, 0.29397259796063463], "id": [1, 2, 3, 4, 5, 7, 8, 16, 17], "count": [1, 1, 1, 1, 1, 5, 1, 3, 1], "members": [[1], [2], [3], [4], [5], [9, 11, 12, 13, 14], [10], [0, 6, 8], [7]]}, "group-clusters": [{"id": 0, "center": [0.04199532248898598, 0.9328668917974231], "members": [1, 3, 4, 16, 17]}, {"id": 1, "center": [-0.08686681998717596, -0.7446120845405175], "members": [2, 5, 7, 8]}], "subgroup-votes": {"0": {"0": {"votes": {"0": {"A": 1, "D": 1, "S": 2}, "1": {"A": 2, "D": 0, "S": 2}, "2": {"A": 0, "D": 0, "S": 0}, "3": {"A": 0, "D": 0, "S": 0}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 0, "D": 0, "S": 0}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 0, "D": 0, "S": 0}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 0, "D": 0, "S": 0}}, "n-members": 2}, "1": {"votes": {"0": {"A": 1, "D": 0, "S": 4}, "1": {"A": 0, "D": 1, "S": 5}, "2": {"A": 0, "D": 0, "S": 0}, "3": {"A": 0, "D": 0, "S": 0}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 0, "D": 0, "S": 0}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 0, "D": 0, "S": 0}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 0, "D": 0, "S": 0}}, "n-members": 5}}, "1": {"0": {"votes": {"0": {"A": 0, "D": 2, "S": 2}, "1": {"A": 0, "D": 1, "S": 1}, "2": {"A": 1, "D": 0, "S": 1}, "3": {"A": 1, "D": 0, "S": 1}, "4": {"A": 1, "D": 0, "S": 1}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 0, "D": 0, "S": 0}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 0, "D": 0, "S": 0}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 0, "D": 0, "S": 0}}, "n-members": 2}, "1": {"votes": {"0": {"A": 0, "D": 0, "S": 1}, "1": {"A": 0, "D": 1, "S": 1}, "2": {"A": 1, "D": 0, "S": 1}, "3": {"A": 0, "D": 0, "S": 0}, "4": {"A": 1, "D": 0, "S": 1}, "5": {"A": 2, "D": 0, "S": 2}, "6": {"A": 1, "D": 0, "S": 1}, "7": {"A": 1, "D": 0, "S": 1}, "8": {"A": 1, "D": 0, "S": 1}, "9": {"A": 1, "D": 0, "S": 1}, "10": {"A": 2, "D": 0, "S": 2}, "11": {"A": 1, "D": 0, "S": 1}, "12": {"A": 1, "D": 0, "S": 1}, "13": {"A": 1, "D": 0, "S": 1}}, "n-members": 6}}}, "lastModTimestamp": null, "subgroup-repness": {"0": {"0": [{"tid": 1, "p-test": 1.7320508075688772, "n-agree": 2, "repness": 5.25, "n-trials": 2, "n-success": 2, "p-success": 0.75, "best-agree": true, "repful-for": "agree", "repness-test": 2.3717082}], "1": [{"tid": 1, "p-test": -0.816496580927726, "repness": 1.142857142857143, "n-trials": 5, "n-success": 1, "p-success": 0.2857142857142857, "repful-for": "disagree", "repness-test": 0.0}]}, "1": {"0": [{"tid": 2, "p-test": 0.0, "n-agree": 1, "repness": 1, "n-trials": 1, "n-success": 0, "p-success": 0.3333333333333333, "best-agree": true, "repful-for": "disagree", "repness-test": 0.0}, {"tid": 0, "p-test": 1.7320508075688772, "repness": 2.25, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "disagree", "repness-test": 1.3693064}], "1": [{"tid": 5, "p-test": 1.7320508075688772, "n-agree": 2, "repness": 1.5, "n-trials": 2, "n-success": 2, "p-success": 0.75, "best-agree": true, "repful-for": "agree", "repness-test": 0.0}]}}, "user-vote-counts": {"0": 2, "1": 2, "2": 2, "3": 2, "4": 2, "5": 2, "6": 2, "7": 1, "8": 2, "9": 1, "10": 4, "11": 7, "12": 3, "13": 1, "14": 1, "15": 1}, "lastVoteTimestamp": 1739363141590, "subgroup-clusters": {"0": [{"id": 0, "center": [1.8708269039033125, 2.286569867150344], "members": [1, 4], "parent-id": 0}, {"id": 1, "center": [-0.6895373100767445, 0.3913857016562545], "members": [3, 16, 17], "parent-id": 0}], "1": [{"id": 0, "center": [0.5879464973635133, -2.25090311422832], "members": [2, 8], "parent-id": 1}, {"id": 1, "center": [-0.3118045924374057, -0.24251507464458338], "members": [5, 7], "parent-id": 1}]}, "comment-priorities": {"0": 6.399968154264931, "1": 6.399968154264931, "2": 15.783634653324425, "3": 12.529726250088048, "4": 15.783634653324425, "5": 15.783634653324425, "6": 12.529726250088048, "7": 12.529726250088048, "8": 12.529726250088048, "9": 12.529726250088048, "10": 15.783634653324425, "11": 12.529726250088048, "12": 12.529726250088048, "13": 12.529726250088048}, "group-aware-consensus": {"0": 0.07500000000000001, "1": 0.08333333333333333, "2": 0.375, "3": 0.3333333333333333, "4": 0.375, "5": 0.375, "6": 0.3333333333333333, "7": 0.3333333333333333, "8": 0.3333333333333333, "9": 0.3333333333333333, "10": 0.375, "11": 0.3333333333333333, "12": 0.3333333333333333, "13": 0.3333333333333333}}	1739363141590	6478	185	1739777950660
5	prod	{"n": 21, "pca": {"comps": [[0.0, 0.0, 0.0, 0.0, 0.13865067337671727, 0.5864775493681436, 0.5320525169796423, 0.3479864995515851, -0.05544535727123839, 0.0, 0.0, 0.0, 0.0, 0.4003191270521332, 0.064438697516735, -0.10216040056102361, 0.0, -0.12964452142373614, -0.002203533443394887, 0.18264643205502146, 0.06729218529879137, 0.0015933078501922584, 0.0048838906800404034], [0.0, 0.0, 0.0, 0.0, -0.27713890896712384, 0.13822626831595972, 0.06111767125759161, -0.14711681801873602, -0.40402228469281914, 0.0, 0.0, 0.0, 0.0, -0.14638243364847858, 0.31047184427173896, -0.29541455060022476, 0.0, -0.4305592774677606, 0.027966903921998028, -0.5337634352156929, 0.05102369604820971, 0.19141929578769679, 0.008534725472199309]], "center": [0.0, 0.0, 0.0, 0.0, -0.8125, -0.375, -0.4375, -0.823529411764706, -0.8125, 0.0, 0.0, 0.0, 0.0, -0.6666666666666665, -0.6470588235294116, -0.6666666666666665, 0.0, -0.6666666666666665, -0.8333333333333333, -0.5, -0.9230769230769231, -0.714285714285714, -0.8571428571428573], "comment-extremity": [0.0, 0.0, 0.0, 0.0, 0.27865614105708947, 1.8060700164225527, 1.444732906491357, 0.31974664568811506, 0.36670937591058733, 0.0, 0.0, 0.0, 0.0, 0.6813967736598573, 0.5367187009763491, 0.4996943060434046, 0.0, 0.7188220956708153, 0.02242337279877595, 1.352779716921864, 0.03115413888715839, 0.26229899824507896, 0.0067369834017072016], "comment-projection": [[0.0, 0.0, 0.0, 0.0, -0.1246772381453618, -1.757904699359334, -1.435294256056225, -0.29450905133589506, 0.04985748604176396, 0.0, 0.0, 0.0, 0.0, -0.6399543629672169, -0.10907193066069562, 0.16331468981493721, 0.0, 0.20725109422291507, 0.0017612958584178376, -0.4379707582350333, -0.024824767963734166, -0.0021832102897981886, -0.0033460452685358474], [0.0, 0.0, 0.0, 0.0, 0.24920840924269247, -0.4143186843372261, -0.16487440562734962, 0.12450837767006728, 0.36330427669699644, 0.0, 0.0, 0.0, 0.0, 0.23400849658353545, -0.5255190557150978, 0.4722528047379396, 0.0, 0.6882965851782117, -0.02235409323976272, 1.2799197542995533, -0.018823157687994658, -0.26228991225970255, -0.005847300780341918]]}, "zid": 5, "tids": [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22], "mod-in": [7, 1, 4, 15, 13, 6, 17, 3, 12, 2, 11, 9, 5, 14, 16, 10, 18, 8], "n-cmts": 23, "in-conv": [0, 7, 20, 1, 4, 15, 13, 6, 17, 3, 12, 2, 11, 9, 5, 14, 16, 10, 18, 8], "mod-out": [0, 1, 3, 12, 2, 11, 9, 16, 10], "repness": {"0": [{"tid": 5, "p-test": 1.9414506867883012, "n-agree": 9, "repness": 4.285714285714286, "n-trials": 12, "n-success": 9, "p-success": 0.7142857142857143, "best-agree": true, "repful-for": "agree", "repness-test": 2.218891}, {"tid": 6, "p-test": 1.9414506867883012, "repness": 4.285714285714286, "n-trials": 12, "n-success": 9, "p-success": 0.7142857142857143, "repful-for": "agree", "repness-test": 2.218891}, {"tid": 20, "p-test": 3.4641016151377544, "repness": 1.846153846153846, "n-trials": 11, "n-success": 11, "p-success": 0.9230769230769231, "repful-for": "agree", "repness-test": 2.0701966}, {"tid": 13, "p-test": 3.356585566713095, "repness": 1.75, "n-trials": 14, "n-success": 13, "p-success": 0.875, "repful-for": "agree", "repness-test": 1.8077538}], "1": [{"tid": 8, "p-test": 2.0, "n-agree": 3, "repness": 1, "n-trials": 3, "n-success": 3, "p-success": 0.8, "best-agree": true, "repful-for": "agree", "repness-test": 0.80178374}, {"tid": 5, "p-test": 1.341640786499874, "repness": 9.333333333333332, "n-trials": 4, "n-success": 3, "p-success": 0.6666666666666667, "repful-for": "disagree", "repness-test": 3.0677555}]}, "consensus": {"agree": [{"tid": 20, "p-test": 3.2071349029490928, "n-trials": 13, "n-success": 12, "p-success": 0.8666666666666667}, {"tid": 7, "p-test": 3.299831645537222, "n-trials": 17, "n-success": 15, "p-success": 0.8421052631578947}, {"tid": 8, "p-test": 3.1529631254723283, "n-trials": 16, "n-success": 14, "p-success": 0.8333333333333333}, {"tid": 13, "p-test": 2.9824045403173027, "n-trials": 18, "n-success": 15, "p-success": 0.8}, {"tid": 4, "p-test": 2.6678918753996634, "n-trials": 16, "n-success": 13, "p-success": 0.7777777777777778}], "disagree": []}, "meta-tids": [1, 3, 12, 2, 11, 9, 16, 10], "votes-base": {"0": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}, "1": {"A": [0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0], "D": [0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0], "S": [1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 2, 1, 1, 1, 1, 1, 1, 0]}, "2": {"A": [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0], "D": [0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0], "S": [1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 2, 1, 1, 1, 1, 1, 1, 0]}, "3": {"A": [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 2, 1, 1, 0, 0, 1, 0, 0], "D": [0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0], "S": [1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 2, 1, 1, 1, 1, 1, 1, 0]}, "4": {"A": [0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 2, 1, 1, 1, 1, 1, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 2, 1, 1, 1, 1, 1, 1, 0]}, "5": {"A": [0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 2, 0, 1, 1, 1, 0, 1, 0], "D": [0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0], "S": [1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 2, 1, 1, 1, 1, 1, 1, 0]}, "6": {"A": [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 2, 0, 1, 1, 1, 1, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0], "S": [1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 2, 1, 1, 1, 1, 1, 1, 0]}, "7": {"A": [0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 2, 1, 1, 1, 1, 1, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 2, 1, 1, 1, 1, 1, 1, 0]}, "8": {"A": [0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 2, 0, 1, 1, 1, 1, 1, 0], "D": [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 2, 0, 1, 1, 1, 1, 1, 0]}, "9": {"A": [0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0], "S": [1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 2, 1, 1, 1, 1, 1, 1, 0]}, "10": {"A": [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0], "S": [1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 2, 1, 1, 1, 1, 1, 1, 0]}, "11": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0], "D": [0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0], "S": [1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 2, 1, 1, 1, 1, 1, 1, 0]}, "12": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0], "D": [0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0], "S": [1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1]}, "13": {"A": [1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 2, 1, 1, 1, 1, 1, 0, 0], "D": [0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0], "S": [1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 0]}, "14": {"A": [1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 2, 1, 1, 1, 1, 0, 1, 0], "D": [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0], "S": [1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 2, 1, 1, 1, 1, 1, 1, 0]}, "15": {"A": [1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 2, 0, 1, 1, 1, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 2, 0, 1, 1, 1, 1, 1, 0]}, "16": {"A": [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0], "D": [0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0], "S": [1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 2, 1, 0, 1, 1, 1, 1, 0]}, "17": {"A": [0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 2, 1, 0, 0, 1, 1, 1, 0], "D": [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 2, 1, 0, 1, 1, 1, 1, 0]}, "18": {"A": [1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 2, 0, 0, 1, 1, 1, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 2, 0, 0, 1, 1, 1, 1, 0]}, "19": {"A": [1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 1, 0, 0, 1, 1, 1, 0], "D": [0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 2, 1, 0, 1, 1, 1, 1, 0]}, "20": {"A": [1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 2, 0, 0, 1, 1, 1, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 2, 1, 0, 1, 1, 1, 1, 0]}, "21": {"A": [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0], "S": [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0]}, "22": {"A": [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0]}}, "group-votes": {"0": {"votes": {"0": {"A": 0, "D": 0, "S": 1}, "1": {"A": 4, "D": 2, "S": 13}, "2": {"A": 2, "D": 4, "S": 13}, "3": {"A": 5, "D": 2, "S": 13}, "4": {"A": 10, "D": 0, "S": 12}, "5": {"A": 9, "D": 0, "S": 12}, "6": {"A": 9, "D": 0, "S": 12}, "7": {"A": 12, "D": 0, "S": 13}, "8": {"A": 11, "D": 1, "S": 13}, "9": {"A": 4, "D": 3, "S": 13}, "10": {"A": 1, "D": 6, "S": 13}, "11": {"A": 0, "D": 6, "S": 13}, "12": {"A": 3, "D": 2, "S": 14}, "13": {"A": 13, "D": 1, "S": 14}, "14": {"A": 11, "D": 1, "S": 14}, "15": {"A": 9, "D": 1, "S": 13}, "16": {"A": 2, "D": 6, "S": 12}, "17": {"A": 7, "D": 1, "S": 10}, "18": {"A": 9, "D": 0, "S": 11}, "19": {"A": 7, "D": 1, "S": 10}, "20": {"A": 11, "D": 0, "S": 11}, "21": {"A": 5, "D": 1, "S": 6}, "22": {"A": 5, "D": 0, "S": 6}}, "n-members": 16}, "1": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 1, "D": 3, "S": 4}, "2": {"A": 1, "D": 2, "S": 4}, "3": {"A": 1, "D": 2, "S": 4}, "4": {"A": 3, "D": 0, "S": 4}, "5": {"A": 0, "D": 3, "S": 4}, "6": {"A": 0, "D": 2, "S": 4}, "7": {"A": 3, "D": 1, "S": 4}, "8": {"A": 3, "D": 0, "S": 3}, "9": {"A": 2, "D": 1, "S": 4}, "10": {"A": 2, "D": 1, "S": 4}, "11": {"A": 1, "D": 2, "S": 4}, "12": {"A": 0, "D": 3, "S": 4}, "13": {"A": 2, "D": 2, "S": 4}, "14": {"A": 2, "D": 1, "S": 3}, "15": {"A": 2, "D": 0, "S": 2}, "16": {"A": 1, "D": 2, "S": 3}, "17": {"A": 2, "D": 0, "S": 2}, "18": {"A": 1, "D": 0, "S": 1}, "19": {"A": 1, "D": 1, "S": 2}, "20": {"A": 1, "D": 0, "S": 2}, "21": {"A": 1, "D": 0, "S": 1}, "22": {"A": 1, "D": 0, "S": 1}}, "n-members": 4}}, "base-clusters": {"x": [0.6312266642793202, 1.2913884106531228, 1.1270058552813622, 0.06323455044128785, -0.43110999902791564, -0.27955861148379973, -0.9895950925084646, -0.4037878234795745, -0.14371226873124962, 3.4742175130881328, -0.20501450298873797, -0.9606000624563347, 1.437523439825253, -1.0060958266250093, -0.8660361608519431, -0.9190380714832285, -0.30265691194225414, -0.2178798607033402, 0.0], "y": [-0.3217588062937116, -0.014509372971959972, 1.1970203597582676, 0.30467014781144963, 0.1707190116528625, -0.4095947209329376, -3.1881562814855626, 0.5583583409362195, 0.21294579458467902, -1.1099188734860523, 0.24438649723113298, 0.4943964832474846, 0.747072559713127, 0.08211533974486483, -0.5478209892488973, 0.41650172343455627, 1.2630957212691627, -0.1351760650684283, 0.0], "id": [0, 3, 5, 6, 7, 9, 12, 13, 14, 15, 17, 18, 19, 20, 21, 22, 23, 24, 25], "count": [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1], "members": [[0], [3], [5], [6], [7], [9], [12], [13], [4], [14], [15], [10, 11], [16], [1], [17], [8], [18], [20], [2]]}, "group-clusters": [{"id": 0, "center": [-0.43695150250109865, -0.022557582491852534], "members": [0, 6, 7, 9, 12, 13, 14, 17, 18, 20, 21, 22, 23, 24, 25]}, {"id": 1, "center": [1.8325338047119677, 0.2049161682533456], "members": [3, 5, 15, 19]}], "subgroup-votes": {"0": {"0": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 1, "D": 0, "S": 1}, "2": {"A": 1, "D": 0, "S": 1}, "3": {"A": 1, "D": 0, "S": 1}, "4": {"A": 0, "D": 0, "S": 1}, "5": {"A": 1, "D": 0, "S": 1}, "6": {"A": 1, "D": 0, "S": 1}, "7": {"A": 1, "D": 0, "S": 1}, "8": {"A": 0, "D": 1, "S": 1}, "9": {"A": 1, "D": 0, "S": 1}, "10": {"A": 0, "D": 1, "S": 1}, "11": {"A": 0, "D": 1, "S": 1}, "12": {"A": 0, "D": 0, "S": 1}, "13": {"A": 1, "D": 0, "S": 1}, "14": {"A": 1, "D": 0, "S": 1}, "15": {"A": 0, "D": 1, "S": 1}, "16": {"A": 0, "D": 1, "S": 1}, "17": {"A": 0, "D": 1, "S": 1}, "18": {"A": 1, "D": 0, "S": 1}, "19": {"A": 0, "D": 1, "S": 1}, "20": {"A": 1, "D": 0, "S": 1}, "21": {"A": 1, "D": 0, "S": 1}, "22": {"A": 1, "D": 0, "S": 1}}, "n-members": 1}, "1": {"votes": {"0": {"A": 0, "D": 0, "S": 1}, "1": {"A": 3, "D": 2, "S": 12}, "2": {"A": 1, "D": 4, "S": 12}, "3": {"A": 4, "D": 2, "S": 12}, "4": {"A": 10, "D": 0, "S": 11}, "5": {"A": 8, "D": 0, "S": 11}, "6": {"A": 8, "D": 0, "S": 11}, "7": {"A": 11, "D": 0, "S": 12}, "8": {"A": 11, "D": 0, "S": 12}, "9": {"A": 3, "D": 3, "S": 12}, "10": {"A": 1, "D": 5, "S": 12}, "11": {"A": 0, "D": 5, "S": 12}, "12": {"A": 3, "D": 2, "S": 13}, "13": {"A": 12, "D": 1, "S": 13}, "14": {"A": 10, "D": 1, "S": 13}, "15": {"A": 9, "D": 0, "S": 12}, "16": {"A": 2, "D": 5, "S": 11}, "17": {"A": 7, "D": 0, "S": 9}, "18": {"A": 8, "D": 0, "S": 10}, "19": {"A": 7, "D": 0, "S": 9}, "20": {"A": 10, "D": 0, "S": 10}, "21": {"A": 4, "D": 1, "S": 5}, "22": {"A": 4, "D": 0, "S": 5}}, "n-members": 15}}, "1": {"0": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 1, "D": 0, "S": 1}, "2": {"A": 0, "D": 1, "S": 1}, "3": {"A": 0, "D": 1, "S": 1}, "4": {"A": 0, "D": 0, "S": 1}, "5": {"A": 0, "D": 1, "S": 1}, "6": {"A": 0, "D": 1, "S": 1}, "7": {"A": 0, "D": 1, "S": 1}, "8": {"A": 1, "D": 0, "S": 1}, "9": {"A": 0, "D": 1, "S": 1}, "10": {"A": 0, "D": 1, "S": 1}, "11": {"A": 0, "D": 1, "S": 1}, "12": {"A": 0, "D": 1, "S": 1}, "13": {"A": 0, "D": 1, "S": 1}, "14": {"A": 1, "D": 0, "S": 1}, "15": {"A": 1, "D": 0, "S": 1}, "16": {"A": 0, "D": 1, "S": 1}, "17": {"A": 1, "D": 0, "S": 1}, "18": {"A": 1, "D": 0, "S": 1}, "19": {"A": 0, "D": 1, "S": 1}, "20": {"A": 1, "D": 0, "S": 1}, "21": {"A": 0, "D": 0, "S": 0}, "22": {"A": 0, "D": 0, "S": 0}}, "n-members": 1}, "1": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 0, "D": 3, "S": 3}, "2": {"A": 1, "D": 1, "S": 3}, "3": {"A": 1, "D": 1, "S": 3}, "4": {"A": 3, "D": 0, "S": 3}, "5": {"A": 0, "D": 2, "S": 3}, "6": {"A": 0, "D": 1, "S": 3}, "7": {"A": 3, "D": 0, "S": 3}, "8": {"A": 2, "D": 0, "S": 2}, "9": {"A": 2, "D": 0, "S": 3}, "10": {"A": 2, "D": 0, "S": 3}, "11": {"A": 1, "D": 1, "S": 3}, "12": {"A": 0, "D": 2, "S": 3}, "13": {"A": 2, "D": 1, "S": 3}, "14": {"A": 1, "D": 1, "S": 2}, "15": {"A": 1, "D": 0, "S": 1}, "16": {"A": 1, "D": 1, "S": 2}, "17": {"A": 1, "D": 0, "S": 1}, "18": {"A": 0, "D": 0, "S": 0}, "19": {"A": 1, "D": 0, "S": 1}, "20": {"A": 0, "D": 0, "S": 1}, "21": {"A": 1, "D": 0, "S": 1}, "22": {"A": 1, "D": 0, "S": 1}}, "n-members": 3}}}, "lastModTimestamp": null, "subgroup-repness": {"0": {"0": [{"tid": 5, "p-test": 0.0, "n-agree": 1, "repness": 4.333333333333333, "n-trials": 1, "n-success": 0, "p-success": 0.3333333333333333, "best-agree": true, "repful-for": "disagree", "repness-test": 1.5590239}, {"tid": 8, "p-test": 1.4142135623730951, "repness": 9.333333333333332, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "repful-for": "disagree", "repness-test": 3.038218}, {"tid": 15, "p-test": 1.4142135623730951, "repness": 9.333333333333332, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "repful-for": "disagree", "repness-test": 3.038218}, {"tid": 17, "p-test": 1.4142135623730951, "repness": 7.333333333333333, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "repful-for": "disagree", "repness-test": 2.6832817}, {"tid": 19, "p-test": 1.4142135623730951, "repness": 7.333333333333333, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "repful-for": "disagree", "repness-test": 2.6832817}], "1": [{"tid": 8, "p-test": 3.05085107923876, "n-agree": 11, "repness": 2.571428571428571, "n-trials": 12, "n-success": 11, "p-success": 0.8571428571428571, "best-agree": true, "repful-for": "agree", "repness-test": 1.6385698}, {"tid": 4, "p-test": 2.886751345948129, "repness": 2.538461538461538, "n-trials": 11, "n-success": 10, "p-success": 0.8461538461538461, "repful-for": "agree", "repness-test": 1.5590239}]}, "1": {"0": [{"tid": 8, "p-test": 0.0, "n-agree": 1, "repness": 1.333333333333333, "n-trials": 1, "n-success": 0, "p-success": 0.3333333333333333, "best-agree": true, "repful-for": "disagree", "repness-test": 0.37267798}, {"tid": 7, "p-test": 1.4142135623730951, "repness": 3.333333333333333, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "repful-for": "disagree", "repness-test": 1.7320508}], "1": [{"tid": 4, "p-test": 2.0, "n-agree": 3, "repness": 2.4, "n-trials": 3, "n-success": 3, "p-success": 0.8, "best-agree": true, "repful-for": "agree", "repness-test": 1.5491934}, {"tid": 7, "p-test": 2.0, "repness": 2.4, "n-trials": 3, "n-success": 3, "p-success": 0.8, "repful-for": "agree", "repness-test": 1.5491934}]}}, "user-vote-counts": {"0": 20, "1": 15, "2": 1, "3": 13, "4": 2, "5": 16, "6": 1, "7": 16, "8": 22, "9": 22, "10": 20, "11": 20, "12": 22, "13": 20, "14": 20, "15": 15, "16": 19, "17": 22, "18": 22, "19": 1, "20": 22}, "lastVoteTimestamp": 1736985982519, "subgroup-clusters": {"0": [{"id": 0, "center": [-0.9895950925084646, -3.1881562814855626], "members": [12], "parent-id": 0}, {"id": 1, "center": [-0.4001085965006076, 0.18848233077439475], "members": [0, 6, 7, 9, 13, 14, 17, 18, 20, 21, 22, 23, 24, 25], "parent-id": 0}], "1": [{"id": 0, "center": [3.4742175130881328, -1.1099188734860523], "members": [15], "parent-id": 1}, {"id": 1, "center": [1.2853059019199127, 0.6431945154998115], "members": [3, 5, 19], "parent-id": 1}]}, "comment-priorities": {"0": 0.7831078906305026, "1": 49.0, "2": 49.0, "3": 49.0, "4": 2.0934787855012105, "5": 4.435397987755234, "6": 2.8686278641106266, "7": 3.0554294304316088, "8": 3.5861108922167495, "9": 49.0, "10": 49.0, "11": 49.0, "12": 49.0, "13": 4.498368492526105, "14": 2.8095586677025417, "15": 2.6213136418283685, "16": 49.0, "17": 5.889229825982472, "18": 2.521410489697851, "19": 8.938079246725009, "20": 3.227358927247741, "21": 12.377969635859559, "22": 6.027997869378607}, "group-aware-consensus": {"0": 0.16666666666666666, "1": 0.1111111111111111, "2": 0.06666666666666667, "3": 0.13333333333333333, "4": 0.5238095238095237, "5": 0.11904761904761903, "6": 0.11904761904761903, "7": 0.5777777777777777, "8": 0.6400000000000001, "9": 0.16666666666666666, "10": 0.06666666666666667, "11": 0.02222222222222222, "12": 0.041666666666666664, "13": 0.4375, "14": 0.45000000000000007, "15": 0.5, "16": 0.08571428571428572, "17": 0.5, "18": 0.5128205128205128, "19": 0.3333333333333333, "20": 0.46153846153846156, "21": 0.5, "22": 0.5}}	1736985982519	4583	616	1737841229506
4	prod	{"n": 2, "pca": {"comps": [[0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.5], [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]], "center": [0.0, 0.0, 0.0, -0.5, -0.5, -0.5, -0.5], "comment-extremity": [0.0, 0.0, 0.0, 0.6614378277661477, 0.6614378277661477, 0.6614378277661477, 0.6614378277661477], "comment-projection": [[0.0, 0.0, 0.0, -0.6614378277661477, -0.6614378277661477, -0.6614378277661477, -0.6614378277661477], [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]]}, "zid": 4, "tids": [0, 1, 2, 3, 4, 5, 6], "mod-in": [0, 1, 4, 6, 3, 2, 5], "n-cmts": 7, "in-conv": [0, 1], "mod-out": [0, 1, 2], "repness": {"0": [{"tid": 3, "p-test": 0.0, "repness": 1, "n-trials": 1, "n-success": 0, "p-success": 0.3333333333333333, "repful-for": "disagree", "repness-test": 0.0}], "1": [{"tid": 3, "p-test": 1.4142135623730951, "n-agree": 1, "repness": 2, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "best-agree": true, "repful-for": "agree", "repness-test": 1.1547005}]}, "consensus": {"agree": [], "disagree": []}, "meta-tids": [0, 1, 2], "votes-base": {"0": {"A": [0, 0], "D": [0, 1], "S": [1, 1]}, "1": {"A": [0, 1], "D": [0, 0], "S": [1, 1]}, "2": {"A": [0, 1], "D": [0, 0], "S": [1, 1]}, "3": {"A": [0, 1], "D": [0, 0], "S": [1, 1]}, "4": {"A": [0, 1], "D": [0, 0], "S": [1, 1]}, "5": {"A": [0, 1], "D": [0, 0], "S": [1, 1]}, "6": {"A": [0, 1], "D": [0, 0], "S": [1, 1]}}, "group-votes": {"0": {"votes": {"0": {"A": 0, "D": 0, "S": 1}, "1": {"A": 0, "D": 0, "S": 1}, "2": {"A": 0, "D": 0, "S": 1}, "3": {"A": 0, "D": 0, "S": 1}, "4": {"A": 0, "D": 0, "S": 1}, "5": {"A": 0, "D": 0, "S": 1}, "6": {"A": 0, "D": 0, "S": 1}}, "n-members": 1}, "1": {"votes": {"0": {"A": 0, "D": 1, "S": 1}, "1": {"A": 1, "D": 0, "S": 1}, "2": {"A": 1, "D": 0, "S": 1}, "3": {"A": 1, "D": 0, "S": 1}, "4": {"A": 1, "D": 0, "S": 1}, "5": {"A": 1, "D": 0, "S": 1}, "6": {"A": 1, "D": 0, "S": 1}}, "n-members": 1}}, "base-clusters": {"x": [1.0, -1.0], "y": [0.0, 0.0], "id": [0, 1], "count": [1, 1], "members": [[0], [1]]}, "group-clusters": [{"id": 0, "center": [1.0, 0.0], "members": [0]}, {"id": 1, "center": [-1.0, 0.0], "members": [1]}], "subgroup-votes": {"0": {"0": {"votes": {"0": {"A": 0, "D": 0, "S": 1}, "1": {"A": 0, "D": 0, "S": 1}, "2": {"A": 0, "D": 0, "S": 1}, "3": {"A": 0, "D": 0, "S": 1}, "4": {"A": 0, "D": 0, "S": 1}, "5": {"A": 0, "D": 0, "S": 1}, "6": {"A": 0, "D": 0, "S": 1}}, "n-members": 1}}, "1": {"0": {"votes": {"0": {"A": 0, "D": 1, "S": 1}, "1": {"A": 1, "D": 0, "S": 1}, "2": {"A": 1, "D": 0, "S": 1}, "3": {"A": 1, "D": 0, "S": 1}, "4": {"A": 1, "D": 0, "S": 1}, "5": {"A": 1, "D": 0, "S": 1}, "6": {"A": 1, "D": 0, "S": 1}}, "n-members": 1}}}, "lastModTimestamp": null, "subgroup-repness": {"0": {"0": [{"tid": 3, "p-test": 0.0, "repness": 0.6666666666666667, "n-trials": 1, "n-success": 0, "p-success": 0.3333333333333333, "repful-for": "disagree", "repness-test": -0.8660254}]}, "1": {"0": [{"tid": 3, "p-test": 1.4142135623730951, "n-agree": 1, "repness": 1.333333333333333, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "best-agree": true, "repful-for": "agree", "repness-test": 0.0}]}}, "user-vote-counts": {"0": 7, "1": 7}, "lastVoteTimestamp": 1736756116183, "subgroup-clusters": {"0": [{"id": 0, "center": [1.0, 0.0], "members": [0], "parent-id": 0}], "1": [{"id": 0, "center": [-1.0, 0.0], "members": [1], "parent-id": 1}]}, "comment-priorities": {"0": 49.0, "1": 49.0, "2": 49.0, "3": 8.606174983279539, "4": 8.606174983279539, "5": 8.606174983279539, "6": 8.606174983279539}, "group-aware-consensus": {"0": 0.1111111111111111, "1": 0.2222222222222222, "2": 0.2222222222222222, "3": 0.2222222222222222, "4": 0.2222222222222222, "5": 0.2222222222222222, "6": 0.2222222222222222}}	1736756116183	4434	259	1737610828807
12	prod	{"n": 61, "pca": {"comps": [[-0.6116794432348979, 0.19943415589519967, -0.03939644604128557, 0.31218148542898266, -0.08266538732601889, 0.0, 0.0855399117109873, -0.08450717320487026, -0.04804556409651596, 0.0, 0.0, 0.27394025476184264, 0.0, 0.19321742739753564, 0.1798271274905498, 0.0, 0.0, 0.05983735054718246, -0.007835431208613623, 0.022398164935423667, 0.05060869251649988, -0.05036639247308185, 0.08334090265967242, -0.06669974436111853, 0.0, 0.0, -0.037536698130376245, -0.2796591148173435, 0.0936590333219432, 0.2791275701678137, -0.0486570313679291, 0.016478043655460194, 0.1867044030622363, -0.008115148559746075, 0.249300522289255, 0.11819549084767801, -0.08808870503331347, 0.0731387352351208, -0.06278617640441374, -0.03133264558122106, 0.008971589251609641, -0.0037732962620439887, 0.0, 0.0, 0.0, 0.0, 0.0], [-0.12343722925474693, -0.141451311506008, 0.022319832170478275, -0.01303628249298855, -0.0607790650533623, 0.0, -0.23308619669546232, -0.28708204107547575, 0.05630238787103927, 0.0, 0.0, -0.08902816771234338, 0.0, -0.19362700152522125, -0.23648822229818053, 0.0, 0.0, 0.017214549151472493, -0.03663251520519197, -0.1487663773100598, -0.048709971180987376, 0.17731083985226556, -0.16043259139956573, -0.06909610142639237, 0.0, 0.0, -0.003862328518845678, -0.516583343738434, -0.13439151483333653, -0.2566999486518257, -0.4439579097857942, -0.011748547168531295, -0.13124794880554383, -0.06057327216173028, -0.0015653434136292505, 0.11881179105124386, -0.14205578522433307, -0.15105074394332985, -0.06329916848802203, -0.015028618182380448, 0.009620801622613466, -0.025253529715177025, 0.0, 0.0, 0.0, 0.0, 0.0]], "center": [0.12962962962962965, -0.7857142857142856, -0.9302325581395354, -0.6818181818181819, -0.7307692307692311, 0.0, -0.7173913043478264, -0.12195121951219516, 0.45238095238095244, 0.0, 0.0, 0.4418604651162789, 0.0, -0.8095238095238098, -0.6764705882352944, 0.0, 0.0, -0.647058823529412, -0.9615384615384609, -0.6666666666666664, -0.7096774193548392, -0.3999999999999995, -0.78125, -0.9130434782608693, 0.0, -1.0, -0.9545454545454539, -0.06521739130434781, 0.5581395348837211, 0.11111111111111101, 0.3023255813953487, -0.7727272727272729, 0.1707317073170731, -0.7619047619047616, -0.21951219512195105, -0.6774193548387096, -0.6129032258064512, -0.6538461538461536, -0.6666666666666664, -0.7222222222222218, -0.75, -0.5, -1.0, -1.0, -1.0, -1.0, -1.0], "comment-extremity": [4.832552264047311, 0.35919395956497735, 0.02165736690147495, 0.6815688908460458, 0.18938237870965322, 0.0, 0.48104730594744155, 1.8014354335241718, 0.7369767224907374, 0.0, 0.0, 2.8472841980658687, 0.0, 0.35719980897421744, 0.658954864605907, 0.0, 0.0, 0.1506574850574357, 0.00987771026149043, 0.3437952074661724, 0.13980572703247662, 0.7582033721529944, 0.2711232325065728, 0.05725193718040703, 0.0, 0.0, 0.011758968841968462, 3.7645374906102522, 1.7498090176878287, 2.8886606771153147, 3.9875219231675176, 0.03153203417885334, 1.8317272314066455, 0.0997570506878846, 1.3339722640142644, 0.3706255841748298, 0.4435856950441786, 0.398270012991551, 0.20374214856086414, 0.06617697468961063, 0.022546222196551214, 0.08752569737590606, 0.0, 0.0, 0.0, 0.0, 0.0], "comment-projection": [[4.737060043111643, -0.29298250464429115, 0.018843378595980613, -0.6809754116903933, 0.15257990001572633, 0.0, -0.16573080782850588, 0.5086993089502156, 0.47839074628942185, 0.0, 0.0, -2.7078712930969706, 0.0, -0.25231084666963205, -0.3988576297774238, 0.0, 0.0, -0.14478501443103567, 0.0020660388465945693, -0.051184694160026245, -0.10072907874320045, 0.20717675415821302, -0.1249842218450785, 0.03976264428477194, 0.0, 0.0, 0.011697210782790122, 1.7922084950606305, -1.000472019497687, -2.126224678355263, 0.4344242988978246, -0.025674494498219274, -1.4985142241195133, 0.013246346561085988, -1.3339459687919721, -0.2613895034117882, 0.23376996228580907, -0.17356635231305886, 0.14348011303617028, 0.059668276617120534, -0.015376529281426549, 0.012934207938778967, 0.0, 0.0, 0.0, 0.0, 0.0], [0.9559411763826835, 0.20780171452691262, -0.010675608844166332, 0.028436625014376896, 0.11218315148415768, 0.0, 0.45159695514452075, 1.7281188136901913, -0.5606041235647837, 0.0, 0.0, 0.88003429738672, 0.0, 0.25284568452728406, 0.5245322723685597, 0.0, 0.0, -0.04165305991204313, 0.009659225822720758, 0.33996363299690324, 0.09694995619713033, -0.7293491249604949, 0.24059665728058927, 0.04119121787944569, 0.0, 0.0, 0.0012035813762947677, 3.310548478491539, 1.4355790945064169, 1.9553846487751116, 3.963786903488722, 0.018305450328175037, 1.0534133900744513, 0.09887367427545504, 0.00837576879988671, -0.26275245222656374, 0.37698789580133524, 0.35846021340314654, 0.14465241188215644, 0.02861972649436318, -0.01648922322590396, 0.08656473858410892, 0.0, 0.0, 0.0, 0.0, 0.0]]}, "zid": 12, "tids": [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46], "mod-in": [0, 7, 20, 27, 1, 39, 4, 21, 31, 32, 40, 33, 13, 22, 36, 41, 29, 6, 28, 25, 34, 17, 3, 2, 23, 35, 19, 11, 14, 26, 38, 30, 18, 37, 8], "n-cmts": 47, "in-conv": [0, 7, 59, 20, 60, 27, 1, 39, 4, 54, 15, 21, 31, 32, 40, 56, 33, 13, 22, 36, 41, 43, 29, 44, 6, 51, 34, 17, 3, 12, 2, 35, 57, 11, 9, 5, 14, 45, 53, 26, 16, 38, 30, 10, 18, 52, 8, 49], "mod-out": [24, 15, 12, 9, 5, 16, 10], "repness": {"0": [{"tid": 0, "p-test": 2.449489742783178, "n-agree": 5, "repness": 2.457142857142857, "n-trials": 5, "n-success": 5, "p-success": 0.8571428571428571, "best-agree": true, "repful-for": "agree", "repness-test": 2.96923}, {"tid": 36, "p-test": 2.449489742783178, "repness": 1.263157894736842, "n-trials": 5, "n-success": 5, "p-success": 0.8571428571428571, "repful-for": "agree", "repness-test": 1.5318834}, {"tid": 4, "p-test": 2.23606797749979, "repness": 1.145833333333333, "n-trials": 4, "n-success": 4, "p-success": 0.8333333333333333, "repful-for": "agree", "repness-test": 1.2881513}, {"tid": 3, "p-test": 1.6329931618554516, "repness": 9.523809523809524, "n-trials": 5, "n-success": 4, "p-success": 0.7142857142857143, "repful-for": "disagree", "repness-test": 4.5115495}, {"tid": 29, "p-test": 1.6329931618554516, "repness": 1.830357142857143, "n-trials": 5, "n-success": 4, "p-success": 0.7142857142857143, "repful-for": "disagree", "repness-test": 1.9871333}], "1": [{"tid": 3, "p-test": 4.123105625617661, "n-agree": 16, "repness": 1.369444444444444, "n-trials": 16, "n-success": 16, "p-success": 0.9444444444444444, "best-agree": true, "repful-for": "agree", "repness-test": 2.4305012}, {"tid": 14, "p-test": 3.605551275463989, "repness": 1.485714285714286, "n-trials": 12, "n-success": 12, "p-success": 0.9285714285714286, "repful-for": "agree", "repness-test": 2.4111543}, {"tid": 1, "p-test": 4.123105625617661, "repness": 1.292397660818713, "n-trials": 16, "n-success": 16, "p-success": 0.9444444444444444, "repful-for": "agree", "repness-test": 2.1817424}, {"tid": 35, "p-test": 3.4641016151377544, "repness": 1.384615384615385, "n-trials": 11, "n-success": 11, "p-success": 0.9230769230769231, "repful-for": "agree", "repness-test": 2.1049392}, {"tid": 0, "p-test": 3.6380343755449944, "repness": 3.160493827160494, "n-trials": 16, "n-success": 15, "p-success": 0.888888888888889, "repful-for": "disagree", "repness-test": 4.316935}], "2": [{"tid": 3, "p-test": 3.1622776601683795, "n-agree": 9, "repness": 1.212121212121212, "n-trials": 9, "n-success": 9, "p-success": 0.9090909090909091, "best-agree": true, "repful-for": "agree", "repness-test": 1.66731}, {"tid": 30, "p-test": 3.4641016151377544, "repness": 2.615384615384615, "n-trials": 11, "n-success": 11, "p-success": 0.9230769230769231, "repful-for": "disagree", "repness-test": 3.7839375}, {"tid": 27, "p-test": 2.3094010767585025, "repness": 2.692307692307692, "n-trials": 11, "n-success": 9, "p-success": 0.7692307692307692, "repful-for": "disagree", "repness-test": 3.2394392}, {"tid": 11, "p-test": 2.886751345948129, "repness": 1.692307692307692, "n-trials": 11, "n-success": 10, "p-success": 0.8461538461538461, "repful-for": "disagree", "repness-test": 2.456701}, {"tid": 32, "p-test": 1.897366596101028, "repness": 2.472727272727273, "n-trials": 9, "n-success": 7, "p-success": 0.7272727272727273, "repful-for": "disagree", "repness-test": 2.790711}], "3": [{"tid": 27, "p-test": 2.8401877872187726, "n-agree": 12, "repness": 3.25, "n-trials": 14, "n-success": 12, "p-success": 0.8125, "best-agree": true, "repful-for": "agree", "repness-test": 3.8847156}, {"tid": 0, "p-test": 1.8073922282301276, "repness": 2.3375, "n-trials": 14, "n-success": 10, "p-success": 0.6875, "repful-for": "agree", "repness-test": 2.785519}, {"tid": 4, "p-test": 3.356585566713095, "repness": 1.293478260869565, "n-trials": 14, "n-success": 13, "p-success": 0.875, "repful-for": "agree", "repness-test": 1.805954}, {"tid": 13, "p-test": 3.7416573867739413, "repness": 1.157333333333333, "n-trials": 13, "n-success": 13, "p-success": 0.9333333333333333, "repful-for": "agree", "repness-test": 1.6224915}, {"tid": 7, "p-test": 1.3867504905630725, "repness": 2.491071428571429, "n-trials": 12, "n-success": 8, "p-success": 0.6428571428571429, "repful-for": "agree", "repness-test": 2.621796}]}, "consensus": {"agree": [{"tid": 23, "p-test": 6.272194634409466, "n-trials": 46, "n-success": 44, "p-success": 0.9375}, {"tid": 25, "p-test": 6.0, "n-trials": 35, "n-success": 35, "p-success": 0.972972972972973}, {"tid": 2, "p-test": 5.728715546977508, "n-trials": 43, "n-success": 40, "p-success": 0.9111111111111111}, {"tid": 18, "p-test": 4.811252243246882, "n-trials": 26, "n-success": 25, "p-success": 0.9285714285714286}, {"tid": 13, "p-test": 5.032452820975954, "n-trials": 42, "n-success": 37, "p-success": 0.8636363636363636}], "disagree": [{"tid": 28, "p-test": 2.110579412044345, "n-trials": 43, "n-success": 28, "p-success": 0.6444444444444444}, {"tid": 8, "p-test": 1.6774842736586513, "n-trials": 42, "n-success": 26, "p-success": 0.6136363636363636}, {"tid": 11, "p-test": 1.5075567228888183, "n-trials": 43, "n-success": 26, "p-success": 0.6}]}, "meta-tids": [], "votes-base": {"0": {"A": [1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0], "D": [0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1], "S": [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]}, "1": {"A": [1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1]}, "2": {"A": [1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1]}, "3": {"A": [1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0], "S": [1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]}, "4": {"A": [1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1]}, "5": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}, "6": {"A": [1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1]}, "7": {"A": [0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0], "D": [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1]}, "8": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0], "D": [1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1], "S": [1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1]}, "9": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}, "10": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}, "11": {"A": [0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1], "S": [1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1]}, "12": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}, "13": {"A": [1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0], "S": [1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]}, "14": {"A": [1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0], "S": [1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "15": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}, "16": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}, "17": {"A": [1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1]}, "18": {"A": [1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1]}, "19": {"A": [1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1]}, "20": {"A": [0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0], "D": [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1]}, "21": {"A": [1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1]}, "22": {"A": [1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1]}, "23": {"A": [1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1]}, "24": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}, "25": {"A": [1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "26": {"A": [0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1]}, "27": {"A": [1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0], "D": [0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1], "S": [1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]}, "28": {"A": [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1], "S": [1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1]}, "29": {"A": [0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0], "D": [1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1], "S": [1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1]}, "30": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0], "D": [1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1], "S": [1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1]}, "31": {"A": [1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0], "S": [1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1]}, "32": {"A": [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0], "D": [1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1], "S": [1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1]}, "33": {"A": [1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1]}, "34": {"A": [0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1], "D": [1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0], "S": [1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1]}, "35": {"A": [0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1], "D": [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0], "S": [1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1]}, "36": {"A": [1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], "S": [1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1]}, "37": {"A": [1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1]}, "38": {"A": [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0], "D": [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], "S": [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1]}, "39": {"A": [0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1]}, "40": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]}, "41": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]}, "42": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]}, "43": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]}, "44": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]}, "45": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]}, "46": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]}}, "group-votes": {"0": {"votes": {"0": {"A": 5, "D": 0, "S": 5}, "1": {"A": 1, "D": 2, "S": 4}, "2": {"A": 4, "D": 0, "S": 4}, "3": {"A": 1, "D": 4, "S": 5}, "4": {"A": 4, "D": 0, "S": 4}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 3, "D": 1, "S": 4}, "7": {"A": 0, "D": 1, "S": 4}, "8": {"A": 1, "D": 2, "S": 4}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 0, "D": 4, "S": 5}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 2, "D": 3, "S": 5}, "14": {"A": 2, "D": 2, "S": 5}, "15": {"A": 0, "D": 0, "S": 0}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 3, "D": 1, "S": 4}, "18": {"A": 4, "D": 0, "S": 4}, "19": {"A": 3, "D": 0, "S": 4}, "20": {"A": 4, "D": 1, "S": 5}, "21": {"A": 3, "D": 0, "S": 4}, "22": {"A": 4, "D": 1, "S": 5}, "23": {"A": 5, "D": 0, "S": 5}, "24": {"A": 0, "D": 0, "S": 0}, "25": {"A": 5, "D": 0, "S": 5}, "26": {"A": 3, "D": 0, "S": 3}, "27": {"A": 2, "D": 1, "S": 5}, "28": {"A": 0, "D": 3, "S": 4}, "29": {"A": 1, "D": 4, "S": 5}, "30": {"A": 0, "D": 3, "S": 4}, "31": {"A": 4, "D": 1, "S": 5}, "32": {"A": 0, "D": 2, "S": 4}, "33": {"A": 3, "D": 0, "S": 4}, "34": {"A": 0, "D": 2, "S": 4}, "35": {"A": 3, "D": 1, "S": 4}, "36": {"A": 5, "D": 0, "S": 5}, "37": {"A": 1, "D": 0, "S": 4}, "38": {"A": 2, "D": 0, "S": 3}, "39": {"A": 2, "D": 0, "S": 3}, "40": {"A": 1, "D": 0, "S": 1}, "41": {"A": 0, "D": 0, "S": 0}, "42": {"A": 0, "D": 0, "S": 0}, "43": {"A": 0, "D": 0, "S": 0}, "44": {"A": 0, "D": 0, "S": 0}, "45": {"A": 0, "D": 0, "S": 0}, "46": {"A": 0, "D": 0, "S": 0}}, "n-members": 5}, "1": {"votes": {"0": {"A": 1, "D": 15, "S": 16}, "1": {"A": 16, "D": 0, "S": 16}, "2": {"A": 13, "D": 0, "S": 16}, "3": {"A": 16, "D": 0, "S": 16}, "4": {"A": 12, "D": 0, "S": 17}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 15, "D": 1, "S": 16}, "7": {"A": 4, "D": 4, "S": 15}, "8": {"A": 1, "D": 9, "S": 15}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 4, "D": 4, "S": 15}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 16, "D": 0, "S": 16}, "14": {"A": 12, "D": 0, "S": 12}, "15": {"A": 0, "D": 0, "S": 0}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 13, "D": 1, "S": 17}, "18": {"A": 10, "D": 0, "S": 10}, "19": {"A": 12, "D": 0, "S": 16}, "20": {"A": 11, "D": 0, "S": 11}, "21": {"A": 6, "D": 2, "S": 10}, "22": {"A": 10, "D": 0, "S": 11}, "23": {"A": 15, "D": 1, "S": 16}, "24": {"A": 0, "D": 0, "S": 0}, "25": {"A": 12, "D": 0, "S": 12}, "26": {"A": 8, "D": 0, "S": 9}, "27": {"A": 4, "D": 7, "S": 14}, "28": {"A": 2, "D": 9, "S": 16}, "29": {"A": 6, "D": 2, "S": 16}, "30": {"A": 3, "D": 7, "S": 16}, "31": {"A": 13, "D": 1, "S": 16}, "32": {"A": 7, "D": 3, "S": 16}, "33": {"A": 14, "D": 1, "S": 16}, "34": {"A": 10, "D": 1, "S": 16}, "35": {"A": 11, "D": 0, "S": 11}, "36": {"A": 8, "D": 0, "S": 11}, "37": {"A": 9, "D": 0, "S": 10}, "38": {"A": 7, "D": 1, "S": 8}, "39": {"A": 6, "D": 1, "S": 7}, "40": {"A": 3, "D": 0, "S": 3}, "41": {"A": 2, "D": 0, "S": 3}, "42": {"A": 0, "D": 0, "S": 0}, "43": {"A": 0, "D": 0, "S": 0}, "44": {"A": 0, "D": 0, "S": 0}, "45": {"A": 0, "D": 0, "S": 0}, "46": {"A": 0, "D": 0, "S": 0}}, "n-members": 17}, "2": {"votes": {"0": {"A": 3, "D": 6, "S": 11}, "1": {"A": 7, "D": 1, "S": 8}, "2": {"A": 9, "D": 0, "S": 9}, "3": {"A": 9, "D": 0, "S": 9}, "4": {"A": 6, "D": 2, "S": 11}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 7, "D": 2, "S": 11}, "7": {"A": 3, "D": 3, "S": 10}, "8": {"A": 3, "D": 8, "S": 12}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 1, "D": 10, "S": 11}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 6, "D": 0, "S": 8}, "14": {"A": 4, "D": 1, "S": 7}, "15": {"A": 0, "D": 0, "S": 0}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 9, "D": 0, "S": 11}, "18": {"A": 3, "D": 0, "S": 4}, "19": {"A": 8, "D": 3, "S": 11}, "20": {"A": 3, "D": 2, "S": 7}, "21": {"A": 4, "D": 0, "S": 4}, "22": {"A": 5, "D": 1, "S": 7}, "23": {"A": 10, "D": 1, "S": 11}, "24": {"A": 0, "D": 0, "S": 0}, "25": {"A": 7, "D": 0, "S": 7}, "26": {"A": 3, "D": 0, "S": 3}, "27": {"A": 1, "D": 9, "S": 11}, "28": {"A": 1, "D": 9, "S": 11}, "29": {"A": 3, "D": 7, "S": 11}, "30": {"A": 0, "D": 11, "S": 11}, "31": {"A": 8, "D": 1, "S": 10}, "32": {"A": 1, "D": 7, "S": 9}, "33": {"A": 7, "D": 1, "S": 9}, "34": {"A": 5, "D": 4, "S": 9}, "35": {"A": 6, "D": 1, "S": 7}, "36": {"A": 4, "D": 3, "S": 7}, "37": {"A": 1, "D": 1, "S": 4}, "38": {"A": 2, "D": 1, "S": 3}, "39": {"A": 2, "D": 0, "S": 3}, "40": {"A": 1, "D": 0, "S": 2}, "41": {"A": 0, "D": 0, "S": 1}, "42": {"A": 1, "D": 0, "S": 1}, "43": {"A": 1, "D": 0, "S": 1}, "44": {"A": 1, "D": 0, "S": 1}, "45": {"A": 1, "D": 0, "S": 1}, "46": {"A": 1, "D": 0, "S": 1}}, "n-members": 12}, "3": {"votes": {"0": {"A": 10, "D": 2, "S": 14}, "1": {"A": 10, "D": 0, "S": 12}, "2": {"A": 13, "D": 0, "S": 13}, "3": {"A": 9, "D": 2, "S": 13}, "4": {"A": 13, "D": 0, "S": 14}, "5": {"A": 1, "D": 0, "S": 1}, "6": {"A": 10, "D": 0, "S": 13}, "7": {"A": 8, "D": 2, "S": 12}, "8": {"A": 2, "D": 7, "S": 11}, "9": {"A": 1, "D": 0, "S": 1}, "10": {"A": 1, "D": 0, "S": 1}, "11": {"A": 2, "D": 8, "S": 12}, "12": {"A": 1, "D": 0, "S": 1}, "13": {"A": 13, "D": 0, "S": 13}, "14": {"A": 8, "D": 0, "S": 10}, "15": {"A": 1, "D": 0, "S": 1}, "16": {"A": 1, "D": 0, "S": 1}, "17": {"A": 10, "D": 3, "S": 14}, "18": {"A": 8, "D": 0, "S": 8}, "19": {"A": 9, "D": 1, "S": 12}, "20": {"A": 7, "D": 0, "S": 8}, "21": {"A": 3, "D": 4, "S": 7}, "22": {"A": 7, "D": 0, "S": 8}, "23": {"A": 13, "D": 0, "S": 13}, "24": {"A": 1, "D": 0, "S": 1}, "25": {"A": 10, "D": 0, "S": 10}, "26": {"A": 7, "D": 0, "S": 7}, "27": {"A": 12, "D": 1, "S": 14}, "28": {"A": 1, "D": 7, "S": 12}, "29": {"A": 3, "D": 6, "S": 12}, "30": {"A": 6, "D": 1, "S": 12}, "31": {"A": 10, "D": 0, "S": 11}, "32": {"A": 1, "D": 4, "S": 12}, "33": {"A": 10, "D": 0, "S": 12}, "34": {"A": 5, "D": 4, "S": 12}, "35": {"A": 4, "D": 2, "S": 8}, "36": {"A": 6, "D": 1, "S": 8}, "37": {"A": 7, "D": 0, "S": 8}, "38": {"A": 5, "D": 0, "S": 7}, "39": {"A": 4, "D": 0, "S": 5}, "40": {"A": 1, "D": 0, "S": 2}, "41": {"A": 1, "D": 0, "S": 2}, "42": {"A": 0, "D": 0, "S": 0}, "43": {"A": 0, "D": 0, "S": 0}, "44": {"A": 0, "D": 0, "S": 0}, "45": {"A": 0, "D": 0, "S": 0}, "46": {"A": 0, "D": 0, "S": 0}}, "n-members": 14}}, "base-clusters": {"x": [2.060405046570264, -0.7275441954392801, -1.1517239915858284, 0.4408743487319334, -2.9939325468968145, 0.5750029545707133, 1.9752815208576364, 0.0851083988410657, 1.12083364320929, -0.44602629107312713, -0.720780854824979, 1.0956146739458572, -1.716990625296464, -0.9914126406477712, -1.1162460343118055, -0.8047708211215421, -1.5772176689186999, -1.9289441189914975, 0.13031345806418856, -0.34421750771800974, -1.9310517930547861, 0.727217628857749, -1.145270793272059, 1.7358834212788292, 1.3171443514016195, 0.7549825285288515, 0.04431757466235809, -0.5585697004127865, -0.14215619920505318, -0.15101380826550012, -1.0616356887539569, 0.37893814817326255, 0.8483693372185332, -1.6533665573299832, 3.642480609123534, 1.6289417137810964, 0.052681913724628895, -0.25456281160257893, 0.5579669261028908, 0.8965891952167336, 1.2701613143835122, -0.5711890079931944, -2.2006558214290246, -1.8917538821850657, 2.532866039269734, 3.9331852355340238, 1.2517198641965057, -0.5243432572504911], "y": [0.3482311489371595, 0.9632023579332308, -0.21061235628874772, 1.2527692797730643, -0.45478703105922186, -0.8148097176240339, 0.9304651768520501, -0.8298406314928173, -1.971570620988054, 2.657221206970554, -0.27166033975213255, 0.8551711779647551, 0.004171624603184964, 1.0152396790851939, -3.2121490513080557, -1.7385668165223815, 0.6161955052044409, 1.0486341462627087, -0.6232626383881356, -0.1327649166101932, -1.138572051104648, 1.1121486666516738, 0.9267301570364432, -0.10865267202945614, 0.7456527599301519, 0.09280224817603021, -1.9442277892002986, 0.35723295301761326, -2.2798264285458667, -0.3524837908907588, 1.012250092535691, 1.296550563072927, 0.01211052480026085, -0.25951294372098993, -3.1061357395550155, -1.2078121547640859, -0.06173551286346063, -0.6925055425171461, 2.2039101213827252, 1.8918317336628558, 1.2687117112126407, 0.14422912064919363, 0.5433267210937652, -0.14692782274721083, 1.0431418666101793, -0.5133801955804896, 1.5618082442819803, -2.350403433793144], "id": [1, 2, 3, 5, 7, 9, 12, 13, 15, 16, 17, 18, 19, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56], "count": [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], "members": [[29], [4], [6], [0], [30], [21], [26], [9], [32], [20], [2], [5], [22], [3], [11], [33], [12], [13], [36], [18], [10], [7], [38], [17], [14], [27], [40], [16], [8], [43], [44], [45], [41], [35], [31], [34], [1], [39], [49], [51], [52], [53], [54], [56], [15], [57], [59], [60]]}, "group-clusters": [{"id": 0, "center": [2.60017920525755, -0.9175499225983774], "members": [1, 31, 43, 44, 54]}, {"id": 1, "center": [-1.3627210232206, 0.23625734683166597], "members": [2, 3, 7, 17, 19, 21, 24, 25, 27, 28, 30, 36, 39, 42, 50, 51, 52]}, {"id": 2, "center": [-0.08206958239039386, -1.4059484978445127], "members": [9, 13, 15, 22, 23, 26, 35, 37, 38, 45, 46, 56]}, {"id": 3, "center": [0.9715499704151209, 1.2088782343815603], "members": [5, 12, 16, 18, 29, 33, 34, 40, 41, 47, 48, 49, 53, 55]}], "subgroup-votes": {"0": {"0": {"votes": {"0": {"A": 3, "D": 0, "S": 3}, "1": {"A": 1, "D": 1, "S": 2}, "2": {"A": 2, "D": 0, "S": 2}, "3": {"A": 1, "D": 2, "S": 3}, "4": {"A": 2, "D": 0, "S": 2}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 2, "D": 0, "S": 2}, "7": {"A": 0, "D": 0, "S": 2}, "8": {"A": 0, "D": 1, "S": 2}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 0, "D": 2, "S": 3}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 2, "D": 1, "S": 3}, "14": {"A": 2, "D": 1, "S": 3}, "15": {"A": 0, "D": 0, "S": 0}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 2, "D": 0, "S": 2}, "18": {"A": 2, "D": 0, "S": 2}, "19": {"A": 2, "D": 0, "S": 2}, "20": {"A": 2, "D": 1, "S": 3}, "21": {"A": 2, "D": 0, "S": 2}, "22": {"A": 3, "D": 0, "S": 3}, "23": {"A": 3, "D": 0, "S": 3}, "24": {"A": 0, "D": 0, "S": 0}, "25": {"A": 3, "D": 0, "S": 3}, "26": {"A": 1, "D": 0, "S": 1}, "27": {"A": 2, "D": 0, "S": 3}, "28": {"A": 0, "D": 1, "S": 2}, "29": {"A": 0, "D": 3, "S": 3}, "30": {"A": 0, "D": 1, "S": 2}, "31": {"A": 2, "D": 1, "S": 3}, "32": {"A": 0, "D": 1, "S": 2}, "33": {"A": 1, "D": 0, "S": 2}, "34": {"A": 0, "D": 1, "S": 2}, "35": {"A": 1, "D": 1, "S": 2}, "36": {"A": 3, "D": 0, "S": 3}, "37": {"A": 1, "D": 0, "S": 2}, "38": {"A": 1, "D": 0, "S": 1}, "39": {"A": 1, "D": 0, "S": 1}, "40": {"A": 1, "D": 0, "S": 1}, "41": {"A": 0, "D": 0, "S": 0}, "42": {"A": 0, "D": 0, "S": 0}, "43": {"A": 0, "D": 0, "S": 0}, "44": {"A": 0, "D": 0, "S": 0}, "45": {"A": 0, "D": 0, "S": 0}, "46": {"A": 0, "D": 0, "S": 0}}, "n-members": 3}, "1": {"votes": {"0": {"A": 2, "D": 0, "S": 2}, "1": {"A": 0, "D": 1, "S": 2}, "2": {"A": 2, "D": 0, "S": 2}, "3": {"A": 0, "D": 2, "S": 2}, "4": {"A": 2, "D": 0, "S": 2}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 1, "D": 1, "S": 2}, "7": {"A": 0, "D": 1, "S": 2}, "8": {"A": 1, "D": 1, "S": 2}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 0, "D": 2, "S": 2}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 0, "D": 2, "S": 2}, "14": {"A": 0, "D": 1, "S": 2}, "15": {"A": 0, "D": 0, "S": 0}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 1, "D": 1, "S": 2}, "18": {"A": 2, "D": 0, "S": 2}, "19": {"A": 1, "D": 0, "S": 2}, "20": {"A": 2, "D": 0, "S": 2}, "21": {"A": 1, "D": 0, "S": 2}, "22": {"A": 1, "D": 1, "S": 2}, "23": {"A": 2, "D": 0, "S": 2}, "24": {"A": 0, "D": 0, "S": 0}, "25": {"A": 2, "D": 0, "S": 2}, "26": {"A": 2, "D": 0, "S": 2}, "27": {"A": 0, "D": 1, "S": 2}, "28": {"A": 0, "D": 2, "S": 2}, "29": {"A": 1, "D": 1, "S": 2}, "30": {"A": 0, "D": 2, "S": 2}, "31": {"A": 2, "D": 0, "S": 2}, "32": {"A": 0, "D": 1, "S": 2}, "33": {"A": 2, "D": 0, "S": 2}, "34": {"A": 0, "D": 1, "S": 2}, "35": {"A": 2, "D": 0, "S": 2}, "36": {"A": 2, "D": 0, "S": 2}, "37": {"A": 0, "D": 0, "S": 2}, "38": {"A": 1, "D": 0, "S": 2}, "39": {"A": 1, "D": 0, "S": 2}, "40": {"A": 0, "D": 0, "S": 0}, "41": {"A": 0, "D": 0, "S": 0}, "42": {"A": 0, "D": 0, "S": 0}, "43": {"A": 0, "D": 0, "S": 0}, "44": {"A": 0, "D": 0, "S": 0}, "45": {"A": 0, "D": 0, "S": 0}, "46": {"A": 0, "D": 0, "S": 0}}, "n-members": 2}}, "1": {"0": {"votes": {"0": {"A": 1, "D": 8, "S": 9}, "1": {"A": 10, "D": 0, "S": 10}, "2": {"A": 8, "D": 0, "S": 10}, "3": {"A": 10, "D": 0, "S": 10}, "4": {"A": 7, "D": 0, "S": 10}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 9, "D": 1, "S": 10}, "7": {"A": 4, "D": 1, "S": 9}, "8": {"A": 0, "D": 5, "S": 9}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 1, "D": 4, "S": 9}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 10, "D": 0, "S": 10}, "14": {"A": 6, "D": 0, "S": 6}, "15": {"A": 0, "D": 0, "S": 0}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 9, "D": 0, "S": 10}, "18": {"A": 6, "D": 0, "S": 6}, "19": {"A": 8, "D": 0, "S": 10}, "20": {"A": 6, "D": 0, "S": 6}, "21": {"A": 4, "D": 0, "S": 6}, "22": {"A": 6, "D": 0, "S": 6}, "23": {"A": 9, "D": 1, "S": 10}, "24": {"A": 0, "D": 0, "S": 0}, "25": {"A": 6, "D": 0, "S": 6}, "26": {"A": 5, "D": 0, "S": 5}, "27": {"A": 4, "D": 3, "S": 8}, "28": {"A": 1, "D": 7, "S": 10}, "29": {"A": 4, "D": 1, "S": 10}, "30": {"A": 3, "D": 3, "S": 10}, "31": {"A": 8, "D": 1, "S": 10}, "32": {"A": 3, "D": 3, "S": 10}, "33": {"A": 9, "D": 0, "S": 10}, "34": {"A": 5, "D": 1, "S": 10}, "35": {"A": 6, "D": 0, "S": 6}, "36": {"A": 5, "D": 0, "S": 6}, "37": {"A": 5, "D": 0, "S": 6}, "38": {"A": 4, "D": 0, "S": 4}, "39": {"A": 4, "D": 0, "S": 4}, "40": {"A": 3, "D": 0, "S": 3}, "41": {"A": 2, "D": 0, "S": 3}, "42": {"A": 0, "D": 0, "S": 0}, "43": {"A": 0, "D": 0, "S": 0}, "44": {"A": 0, "D": 0, "S": 0}, "45": {"A": 0, "D": 0, "S": 0}, "46": {"A": 0, "D": 0, "S": 0}}, "n-members": 10}, "1": {"votes": {"0": {"A": 0, "D": 7, "S": 7}, "1": {"A": 6, "D": 0, "S": 6}, "2": {"A": 5, "D": 0, "S": 6}, "3": {"A": 6, "D": 0, "S": 6}, "4": {"A": 5, "D": 0, "S": 7}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 6, "D": 0, "S": 6}, "7": {"A": 0, "D": 3, "S": 6}, "8": {"A": 1, "D": 4, "S": 6}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 3, "D": 0, "S": 6}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 6, "D": 0, "S": 6}, "14": {"A": 6, "D": 0, "S": 6}, "15": {"A": 0, "D": 0, "S": 0}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 4, "D": 1, "S": 7}, "18": {"A": 4, "D": 0, "S": 4}, "19": {"A": 4, "D": 0, "S": 6}, "20": {"A": 5, "D": 0, "S": 5}, "21": {"A": 2, "D": 2, "S": 4}, "22": {"A": 4, "D": 0, "S": 5}, "23": {"A": 6, "D": 0, "S": 6}, "24": {"A": 0, "D": 0, "S": 0}, "25": {"A": 6, "D": 0, "S": 6}, "26": {"A": 3, "D": 0, "S": 4}, "27": {"A": 0, "D": 4, "S": 6}, "28": {"A": 1, "D": 2, "S": 6}, "29": {"A": 2, "D": 1, "S": 6}, "30": {"A": 0, "D": 4, "S": 6}, "31": {"A": 5, "D": 0, "S": 6}, "32": {"A": 4, "D": 0, "S": 6}, "33": {"A": 5, "D": 1, "S": 6}, "34": {"A": 5, "D": 0, "S": 6}, "35": {"A": 5, "D": 0, "S": 5}, "36": {"A": 3, "D": 0, "S": 5}, "37": {"A": 4, "D": 0, "S": 4}, "38": {"A": 3, "D": 1, "S": 4}, "39": {"A": 2, "D": 1, "S": 3}, "40": {"A": 0, "D": 0, "S": 0}, "41": {"A": 0, "D": 0, "S": 0}, "42": {"A": 0, "D": 0, "S": 0}, "43": {"A": 0, "D": 0, "S": 0}, "44": {"A": 0, "D": 0, "S": 0}, "45": {"A": 0, "D": 0, "S": 0}, "46": {"A": 0, "D": 0, "S": 0}}, "n-members": 7}}, "2": {"0": {"votes": {"0": {"A": 2, "D": 1, "S": 3}, "1": {"A": 3, "D": 0, "S": 3}, "2": {"A": 3, "D": 0, "S": 3}, "3": {"A": 3, "D": 0, "S": 3}, "4": {"A": 1, "D": 1, "S": 3}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 2, "D": 0, "S": 3}, "7": {"A": 2, "D": 0, "S": 3}, "8": {"A": 0, "D": 3, "S": 4}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 0, "D": 3, "S": 3}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 2, "D": 0, "S": 3}, "14": {"A": 2, "D": 0, "S": 2}, "15": {"A": 0, "D": 0, "S": 0}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 2, "D": 0, "S": 3}, "18": {"A": 1, "D": 0, "S": 1}, "19": {"A": 3, "D": 0, "S": 3}, "20": {"A": 0, "D": 1, "S": 2}, "21": {"A": 1, "D": 0, "S": 1}, "22": {"A": 2, "D": 0, "S": 2}, "23": {"A": 3, "D": 0, "S": 3}, "24": {"A": 0, "D": 0, "S": 0}, "25": {"A": 2, "D": 0, "S": 2}, "26": {"A": 1, "D": 0, "S": 1}, "27": {"A": 1, "D": 2, "S": 3}, "28": {"A": 0, "D": 3, "S": 3}, "29": {"A": 1, "D": 1, "S": 3}, "30": {"A": 0, "D": 3, "S": 3}, "31": {"A": 3, "D": 0, "S": 3}, "32": {"A": 0, "D": 2, "S": 3}, "33": {"A": 2, "D": 0, "S": 3}, "34": {"A": 2, "D": 1, "S": 3}, "35": {"A": 2, "D": 0, "S": 2}, "36": {"A": 1, "D": 1, "S": 2}, "37": {"A": 0, "D": 0, "S": 1}, "38": {"A": 1, "D": 0, "S": 1}, "39": {"A": 1, "D": 0, "S": 1}, "40": {"A": 0, "D": 0, "S": 1}, "41": {"A": 0, "D": 0, "S": 0}, "42": {"A": 0, "D": 0, "S": 0}, "43": {"A": 0, "D": 0, "S": 0}, "44": {"A": 0, "D": 0, "S": 0}, "45": {"A": 0, "D": 0, "S": 0}, "46": {"A": 0, "D": 0, "S": 0}}, "n-members": 4}, "1": {"votes": {"0": {"A": 0, "D": 1, "S": 2}, "1": {"A": 1, "D": 0, "S": 1}, "2": {"A": 2, "D": 0, "S": 2}, "3": {"A": 2, "D": 0, "S": 2}, "4": {"A": 1, "D": 0, "S": 2}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 2, "D": 0, "S": 2}, "7": {"A": 0, "D": 0, "S": 2}, "8": {"A": 1, "D": 1, "S": 2}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 0, "D": 2, "S": 2}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 1, "D": 0, "S": 1}, "14": {"A": 1, "D": 0, "S": 1}, "15": {"A": 0, "D": 0, "S": 0}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 2, "D": 0, "S": 2}, "18": {"A": 0, "D": 0, "S": 0}, "19": {"A": 2, "D": 0, "S": 2}, "20": {"A": 1, "D": 0, "S": 1}, "21": {"A": 0, "D": 0, "S": 0}, "22": {"A": 1, "D": 0, "S": 1}, "23": {"A": 2, "D": 0, "S": 2}, "24": {"A": 0, "D": 0, "S": 0}, "25": {"A": 1, "D": 0, "S": 1}, "26": {"A": 0, "D": 0, "S": 0}, "27": {"A": 0, "D": 1, "S": 2}, "28": {"A": 1, "D": 0, "S": 2}, "29": {"A": 0, "D": 2, "S": 2}, "30": {"A": 0, "D": 2, "S": 2}, "31": {"A": 1, "D": 0, "S": 2}, "32": {"A": 1, "D": 1, "S": 2}, "33": {"A": 2, "D": 0, "S": 2}, "34": {"A": 0, "D": 2, "S": 2}, "35": {"A": 1, "D": 0, "S": 1}, "36": {"A": 1, "D": 0, "S": 1}, "37": {"A": 0, "D": 0, "S": 0}, "38": {"A": 0, "D": 0, "S": 0}, "39": {"A": 0, "D": 0, "S": 0}, "40": {"A": 0, "D": 0, "S": 0}, "41": {"A": 0, "D": 0, "S": 0}, "42": {"A": 0, "D": 0, "S": 0}, "43": {"A": 0, "D": 0, "S": 0}, "44": {"A": 0, "D": 0, "S": 0}, "45": {"A": 0, "D": 0, "S": 0}, "46": {"A": 0, "D": 0, "S": 0}}, "n-members": 2}, "2": {"votes": {"0": {"A": 1, "D": 4, "S": 6}, "1": {"A": 3, "D": 1, "S": 4}, "2": {"A": 4, "D": 0, "S": 4}, "3": {"A": 4, "D": 0, "S": 4}, "4": {"A": 4, "D": 1, "S": 6}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 3, "D": 2, "S": 6}, "7": {"A": 1, "D": 3, "S": 5}, "8": {"A": 2, "D": 4, "S": 6}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 1, "D": 5, "S": 6}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 3, "D": 0, "S": 4}, "14": {"A": 1, "D": 1, "S": 4}, "15": {"A": 0, "D": 0, "S": 0}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 5, "D": 0, "S": 6}, "18": {"A": 2, "D": 0, "S": 3}, "19": {"A": 3, "D": 3, "S": 6}, "20": {"A": 2, "D": 1, "S": 4}, "21": {"A": 3, "D": 0, "S": 3}, "22": {"A": 2, "D": 1, "S": 4}, "23": {"A": 5, "D": 1, "S": 6}, "24": {"A": 0, "D": 0, "S": 0}, "25": {"A": 4, "D": 0, "S": 4}, "26": {"A": 2, "D": 0, "S": 2}, "27": {"A": 0, "D": 6, "S": 6}, "28": {"A": 0, "D": 6, "S": 6}, "29": {"A": 2, "D": 4, "S": 6}, "30": {"A": 0, "D": 6, "S": 6}, "31": {"A": 4, "D": 1, "S": 5}, "32": {"A": 0, "D": 4, "S": 4}, "33": {"A": 3, "D": 1, "S": 4}, "34": {"A": 3, "D": 1, "S": 4}, "35": {"A": 3, "D": 1, "S": 4}, "36": {"A": 2, "D": 2, "S": 4}, "37": {"A": 1, "D": 1, "S": 3}, "38": {"A": 1, "D": 1, "S": 2}, "39": {"A": 1, "D": 0, "S": 2}, "40": {"A": 1, "D": 0, "S": 1}, "41": {"A": 0, "D": 0, "S": 1}, "42": {"A": 1, "D": 0, "S": 1}, "43": {"A": 1, "D": 0, "S": 1}, "44": {"A": 1, "D": 0, "S": 1}, "45": {"A": 1, "D": 0, "S": 1}, "46": {"A": 1, "D": 0, "S": 1}}, "n-members": 6}}, "3": {"0": {"votes": {"0": {"A": 7, "D": 1, "S": 10}, "1": {"A": 8, "D": 0, "S": 9}, "2": {"A": 10, "D": 0, "S": 10}, "3": {"A": 8, "D": 0, "S": 10}, "4": {"A": 9, "D": 0, "S": 10}, "5": {"A": 1, "D": 0, "S": 1}, "6": {"A": 8, "D": 0, "S": 9}, "7": {"A": 5, "D": 2, "S": 9}, "8": {"A": 1, "D": 5, "S": 8}, "9": {"A": 1, "D": 0, "S": 1}, "10": {"A": 1, "D": 0, "S": 1}, "11": {"A": 1, "D": 6, "S": 8}, "12": {"A": 1, "D": 0, "S": 1}, "13": {"A": 10, "D": 0, "S": 10}, "14": {"A": 6, "D": 0, "S": 7}, "15": {"A": 1, "D": 0, "S": 1}, "16": {"A": 1, "D": 0, "S": 1}, "17": {"A": 7, "D": 2, "S": 10}, "18": {"A": 5, "D": 0, "S": 5}, "19": {"A": 5, "D": 1, "S": 8}, "20": {"A": 5, "D": 0, "S": 5}, "21": {"A": 2, "D": 2, "S": 4}, "22": {"A": 5, "D": 0, "S": 5}, "23": {"A": 9, "D": 0, "S": 9}, "24": {"A": 1, "D": 0, "S": 1}, "25": {"A": 7, "D": 0, "S": 7}, "26": {"A": 5, "D": 0, "S": 5}, "27": {"A": 8, "D": 1, "S": 10}, "28": {"A": 1, "D": 5, "S": 8}, "29": {"A": 1, "D": 4, "S": 8}, "30": {"A": 4, "D": 1, "S": 8}, "31": {"A": 7, "D": 0, "S": 8}, "32": {"A": 0, "D": 3, "S": 9}, "33": {"A": 7, "D": 0, "S": 9}, "34": {"A": 3, "D": 4, "S": 9}, "35": {"A": 3, "D": 0, "S": 5}, "36": {"A": 3, "D": 1, "S": 5}, "37": {"A": 4, "D": 0, "S": 5}, "38": {"A": 3, "D": 0, "S": 5}, "39": {"A": 3, "D": 0, "S": 4}, "40": {"A": 1, "D": 0, "S": 2}, "41": {"A": 1, "D": 0, "S": 2}, "42": {"A": 0, "D": 0, "S": 0}, "43": {"A": 0, "D": 0, "S": 0}, "44": {"A": 0, "D": 0, "S": 0}, "45": {"A": 0, "D": 0, "S": 0}, "46": {"A": 0, "D": 0, "S": 0}}, "n-members": 10}, "1": {"votes": {"0": {"A": 2, "D": 0, "S": 2}, "1": {"A": 0, "D": 0, "S": 1}, "2": {"A": 1, "D": 0, "S": 1}, "3": {"A": 0, "D": 1, "S": 1}, "4": {"A": 2, "D": 0, "S": 2}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 0, "D": 0, "S": 2}, "7": {"A": 1, "D": 0, "S": 1}, "8": {"A": 1, "D": 0, "S": 1}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 0, "D": 1, "S": 2}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 1, "D": 0, "S": 1}, "14": {"A": 0, "D": 0, "S": 1}, "15": {"A": 0, "D": 0, "S": 0}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 1, "D": 1, "S": 2}, "18": {"A": 1, "D": 0, "S": 1}, "19": {"A": 2, "D": 0, "S": 2}, "20": {"A": 1, "D": 0, "S": 1}, "21": {"A": 1, "D": 0, "S": 1}, "22": {"A": 0, "D": 0, "S": 1}, "23": {"A": 2, "D": 0, "S": 2}, "24": {"A": 0, "D": 0, "S": 0}, "25": {"A": 1, "D": 0, "S": 1}, "26": {"A": 1, "D": 0, "S": 1}, "27": {"A": 2, "D": 0, "S": 2}, "28": {"A": 0, "D": 1, "S": 2}, "29": {"A": 0, "D": 2, "S": 2}, "30": {"A": 1, "D": 0, "S": 2}, "31": {"A": 1, "D": 0, "S": 1}, "32": {"A": 1, "D": 0, "S": 1}, "33": {"A": 1, "D": 0, "S": 1}, "34": {"A": 1, "D": 0, "S": 1}, "35": {"A": 0, "D": 1, "S": 1}, "36": {"A": 1, "D": 0, "S": 1}, "37": {"A": 1, "D": 0, "S": 1}, "38": {"A": 1, "D": 0, "S": 1}, "39": {"A": 1, "D": 0, "S": 1}, "40": {"A": 0, "D": 0, "S": 0}, "41": {"A": 0, "D": 0, "S": 0}, "42": {"A": 0, "D": 0, "S": 0}, "43": {"A": 0, "D": 0, "S": 0}, "44": {"A": 0, "D": 0, "S": 0}, "45": {"A": 0, "D": 0, "S": 0}, "46": {"A": 0, "D": 0, "S": 0}}, "n-members": 2}, "2": {"votes": {"0": {"A": 1, "D": 1, "S": 2}, "1": {"A": 2, "D": 0, "S": 2}, "2": {"A": 2, "D": 0, "S": 2}, "3": {"A": 1, "D": 1, "S": 2}, "4": {"A": 2, "D": 0, "S": 2}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 2, "D": 0, "S": 2}, "7": {"A": 2, "D": 0, "S": 2}, "8": {"A": 0, "D": 2, "S": 2}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 1, "D": 1, "S": 2}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 2, "D": 0, "S": 2}, "14": {"A": 2, "D": 0, "S": 2}, "15": {"A": 0, "D": 0, "S": 0}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 2, "D": 0, "S": 2}, "18": {"A": 2, "D": 0, "S": 2}, "19": {"A": 2, "D": 0, "S": 2}, "20": {"A": 1, "D": 0, "S": 2}, "21": {"A": 0, "D": 2, "S": 2}, "22": {"A": 2, "D": 0, "S": 2}, "23": {"A": 2, "D": 0, "S": 2}, "24": {"A": 0, "D": 0, "S": 0}, "25": {"A": 2, "D": 0, "S": 2}, "26": {"A": 1, "D": 0, "S": 1}, "27": {"A": 2, "D": 0, "S": 2}, "28": {"A": 0, "D": 1, "S": 2}, "29": {"A": 2, "D": 0, "S": 2}, "30": {"A": 1, "D": 0, "S": 2}, "31": {"A": 2, "D": 0, "S": 2}, "32": {"A": 0, "D": 1, "S": 2}, "33": {"A": 2, "D": 0, "S": 2}, "34": {"A": 1, "D": 0, "S": 2}, "35": {"A": 1, "D": 1, "S": 2}, "36": {"A": 2, "D": 0, "S": 2}, "37": {"A": 2, "D": 0, "S": 2}, "38": {"A": 1, "D": 0, "S": 1}, "39": {"A": 0, "D": 0, "S": 0}, "40": {"A": 0, "D": 0, "S": 0}, "41": {"A": 0, "D": 0, "S": 0}, "42": {"A": 0, "D": 0, "S": 0}, "43": {"A": 0, "D": 0, "S": 0}, "44": {"A": 0, "D": 0, "S": 0}, "45": {"A": 0, "D": 0, "S": 0}, "46": {"A": 0, "D": 0, "S": 0}}, "n-members": 2}}}, "lastModTimestamp": null, "subgroup-repness": {"0": {"0": [{"tid": 22, "p-test": 2.0, "n-agree": 3, "repness": 1.6, "n-trials": 3, "n-success": 3, "p-success": 0.8, "best-agree": true, "repful-for": "agree", "repness-test": 1.2472191}], "1": [{"tid": 0, "p-test": -0.5773502691896258, "n-agree": 2, "repness": 1.25, "n-trials": 2, "n-success": 0, "p-success": 0.25, "best-agree": true, "repful-for": "disagree", "repness-test": 0.24152295}, {"tid": 13, "p-test": 1.7320508075688772, "repness": 1.875, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "disagree", "repness-test": 1.4491377}]}, "1": {"0": [{"tid": 17, "p-test": 2.7136021011998723, "n-agree": 9, "repness": 1.5, "n-trials": 10, "n-success": 9, "p-success": 0.8333333333333333, "best-agree": true, "repful-for": "agree", "repness-test": 1.4996843}], "1": [{"tid": 34, "p-test": 1.8898223650461359, "repness": 1.5, "n-trials": 6, "n-success": 5, "p-success": 0.75, "repful-for": "agree", "repness-test": 1.3675269}, {"tid": 1, "p-test": -1.8898223650461359, "n-agree": 6, "repness": 1.5, "n-trials": 6, "n-success": 0, "p-success": 0.125, "best-agree": true, "repful-for": "disagree", "repness-test": 0.34188172}]}, "2": {"0": [{"tid": 14, "p-test": 1.7320508075688772, "n-agree": 2, "repness": 1.75, "n-trials": 2, "n-success": 2, "p-success": 0.75, "best-agree": true, "repful-for": "agree", "repness-test": 1.5}, {"tid": 19, "p-test": 2.0, "repness": 1.333333333333333, "n-trials": 3, "n-success": 3, "p-success": 0.8, "repful-for": "agree", "repness-test": 1.3165612}], "1": [{"tid": 20, "p-test": 1.4142135623730951, "repness": 1.777777777777778, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "repful-for": "agree", "repness-test": 1.4342743}, {"tid": 6, "p-test": 1.7320508075688772, "repness": 1.375, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "agree", "repness-test": 1.3165612}, {"tid": 2, "p-test": -0.5773502691896258, "n-agree": 2, "repness": 2.25, "n-trials": 2, "n-success": 0, "p-success": 0.25, "best-agree": true, "repful-for": "disagree", "repness-test": 0.7978559}, {"tid": 34, "p-test": 1.7320508075688772, "repness": 2.25, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "disagree", "repness-test": 1.8540496}, {"tid": 29, "p-test": 1.7320508075688772, "repness": 1.375, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "disagree", "repness-test": 1.3165612}], "2": [{"tid": 2, "p-test": -1.3416407864998738, "n-agree": 4, "repness": 1.166666666666667, "n-trials": 4, "n-success": 0, "p-success": 0.1666666666666667, "best-agree": true, "repful-for": "disagree", "repness-test": 0.14272481}, {"tid": 27, "p-test": 2.6457513110645907, "repness": 1.53125, "n-trials": 6, "n-success": 6, "p-success": 0.875, "repful-for": "disagree", "repness-test": 1.660595}, {"tid": 28, "p-test": 2.6457513110645907, "repness": 1.53125, "n-trials": 6, "n-success": 6, "p-success": 0.875, "repful-for": "disagree", "repness-test": 1.660595}, {"tid": 32, "p-test": 2.23606797749979, "repness": 1.458333333333333, "n-trials": 4, "n-success": 4, "p-success": 0.8333333333333333, "repful-for": "disagree", "repness-test": 1.4272481}]}, "3": {"0": [{"tid": 6, "p-test": 2.529822128134704, "n-agree": 8, "repness": 1.636363636363636, "n-trials": 9, "n-success": 8, "p-success": 0.8181818181818182, "best-agree": true, "repful-for": "agree", "repness-test": 1.3693064}, {"tid": 20, "p-test": 2.449489742783178, "repness": 1.428571428571429, "n-trials": 5, "n-success": 5, "p-success": 0.8571428571428571, "repful-for": "agree", "repness-test": 1.2909944}, {"tid": 22, "p-test": 2.449489742783178, "repness": 1.428571428571429, "n-trials": 5, "n-success": 5, "p-success": 0.8571428571428571, "repful-for": "agree", "repness-test": 1.2909944}], "1": [{"tid": 32, "p-test": 1.4142135623730951, "n-agree": 1, "repness": 8.666666666666668, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "best-agree": true, "repful-for": "agree", "repness-test": 2.924988}, {"tid": 8, "p-test": 1.4142135623730951, "repness": 4, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "repful-for": "agree", "repness-test": 2.3061182}, {"tid": 3, "p-test": 1.4142135623730951, "repness": 4.666666666666667, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "repful-for": "disagree", "repness-test": 2.5191574}, {"tid": 35, "p-test": 1.4142135623730951, "repness": 3, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "repful-for": "disagree", "repness-test": 1.9364917}, {"tid": 29, "p-test": 1.7320508075688772, "repness": 1.8, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "disagree", "repness-test": 1.6922282}], "2": [{"tid": 29, "p-test": 1.7320508075688772, "repness": 4.5, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "agree", "repness-test": 2.6215887}, {"tid": 1, "p-test": -0.5773502691896258, "n-agree": 2, "repness": 3, "n-trials": 2, "n-success": 0, "p-success": 0.25, "best-agree": true, "repful-for": "disagree", "repness-test": 1.063632}, {"tid": 21, "p-test": 1.7320508075688772, "repness": 1.75, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "disagree", "repness-test": 1.5}, {"tid": 8, "p-test": 1.7320508075688772, "repness": 1.375, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "disagree", "repness-test": 1.3165612}]}}, "user-vote-counts": {"0": 42, "1": 1, "2": 35, "3": 35, "4": 21, "5": 35, "6": 3, "7": 21, "8": 30, "9": 19, "10": 33, "11": 33, "12": 31, "13": 19, "14": 23, "15": 33, "16": 33, "17": 34, "18": 35, "19": 3, "20": 30, "21": 34, "22": 32, "23": 3, "24": 3, "25": 3, "26": 11, "27": 33, "28": 5, "29": 30, "30": 33, "31": 33, "32": 12, "33": 14, "34": 33, "35": 33, "36": 27, "37": 2, "38": 17, "39": 27, "40": 27, "41": 32, "42": 2, "43": 21, "44": 21, "45": 21, "46": 2, "47": 4, "48": 1, "49": 32, "50": 3, "51": 21, "52": 10, "53": 30, "54": 23, "55": 3, "56": 27, "57": 13, "58": 3, "59": 24, "60": 40}, "lastVoteTimestamp": 1739596748921, "subgroup-clusters": {"0": [{"id": 0, "center": [2.576491234461039, -0.09126723955759541], "members": [1, 31, 54], "parent-id": 0}, {"id": 1, "center": [2.635711161452315, -2.1569739471595506], "members": [43, 44], "parent-id": 0}], "1": [{"id": 0, "center": [-0.9626782176972233, 0.567928875536219], "members": [2, 17, 21, 24, 25, 27, 30, 36, 39, 50], "parent-id": 1}, {"id": 1, "center": [-1.9342107453968524, -0.23755912274626684], "members": [3, 7, 19, 28, 42, 51, 52], "parent-id": 1}], "2": [{"id": 0, "center": [0.15174612952350766, -0.46307291494159725], "members": [9, 26, 38, 45], "parent-id": 2}, {"id": 1, "center": [-0.08472720638075662, -0.7611730870049818], "members": [13, 46], "parent-id": 2}, {"id": 2, "center": [-0.23706084900287397, -2.2494573567263], "members": [15, 22, 23, 35, 37, 56], "parent-id": 2}], "3": [{"id": 0, "center": [0.8981611390654559, 1.008955690952634], "members": [5, 18, 29, 33, 34, 40, 41, 48, 49, 55], "parent-id": 3}, {"id": 1, "center": [2.254073780063685, 0.9868035217311146], "members": [12, 53], "parent-id": 3}, {"id": 2, "center": [0.05597031751488182, 2.4305656641766396], "members": [16, 47], "parent-id": 3}]}, "comment-priorities": {"0": 4.869500905091996, "1": 1.1168553143697866, "2": 0.7470864232331345, "3": 1.642134465986389, "4": 0.5123720900845381, "5": 12.529726250088048, "6": 1.0526477261848122, "7": 0.4191679550336146, "8": 0.062408844519720494, "9": 12.529726250088048, "10": 12.529726250088048, "11": 0.2781775968922634, "12": 12.529726250088048, "13": 1.2500944390525894, "14": 1.2349561360258976, "15": 12.529726250088048, "16": 12.529726250088048, "17": 0.5582582103801196, "18": 1.124177709739365, "19": 0.6838696167779293, "20": 0.7657196274279865, "21": 1.3894940823582442, "22": 1.0269853657711676, "23": 0.9679509024014737, "24": 12.529726250088048, "25": 1.0263241102373835, "26": 1.374307074248106, "27": 3.034512238040861, "28": 0.052293626134268405, "29": 0.7469718912799679, "30": 0.6470612576524695, "31": 0.586435147520769, "32": 0.16729759898726643, "33": 0.6259984018780449, "34": 0.759224309629046, "35": 1.1918423493314882, "36": 0.9756388909408698, "37": 0.6810096810078742, "38": 1.1128356922780063, "39": 1.1273234050937997, "40": 3.324496995193117, "41": 1.4850530636825718, "42": 12.529726250088048, "43": 12.529726250088048, "44": 12.529726250088048, "45": 12.529726250088048, "46": 12.529726250088048}, "group-aware-consensus": {"0": 0.020146520146520148, "1": 0.19788359788359786, "2": 0.5499438832772165, "3": 0.16354016354016354, "4": 0.268640350877193, "5": 0.08333333333333333, "6": 0.26742640075973406, "7": 0.010504201680672266, "8": 0.002585649644473174, "9": 0.08333333333333333, "10": 0.08333333333333333, "11": 0.001385169452396343, "12": 0.08333333333333333, "13": 0.2644444444444444, "14": 0.16581632653061223, "15": 0.08333333333333333, "16": 0.08333333333333333, "17": 0.25978407557354927, "18": 0.45833333333333326, "19": 0.23809523809523805, "20": 0.2344322344322344, "21": 0.14403292181069954, "22": 0.3223443223443223, "23": 0.6017094017094018, "24": 0.08333333333333333, "25": 0.6485260770975055, "26": 0.46545454545454557, "27": 0.016741071428571428, "28": 0.0006105006105006105, "29": 0.009768009768009766, "30": 0.0014245014245014246, "31": 0.3525641025641025, "32": 0.0019240019240019239, "33": 0.3174603174603174, "34": 0.0238095238095238, "35": 0.2393162393162393, "36": 0.23076923076923078, "37": 0.07407407407407406, "38": 0.19200000000000006, "39": 0.19999999999999998, "40": 0.13333333333333333, "41": 0.05, "42": 0.08333333333333333, "43": 0.08333333333333333, "44": 0.08333333333333333, "45": 0.08333333333333333, "46": 0.08333333333333333}}	1739596748921	6482	1259	1739777951216
3	prod	{"n": 8, "pca": {"comps": [[0.1721833287540316, -0.7129593595149052, 0.0, 0.30850835843511126, 0.2976082157552215, 0.4307780942011569, -0.3045062055629788, 0.0, 0.0], [0.6816581786304995, -0.2645880084790307, 0.0, -0.5803731752233479, 0.1457528654251131, -0.2320650606150831, 0.2310934889141323, 0.0, 0.0]], "center": [-0.2857142857142857, -0.14285714285714285, -1.0, -0.2, 0.0, -0.3333333333333333, 0.0, -1.0, -1.0], "comment-extremity": [1.5065748321518846, 1.9554997663130034, 0.0, 1.5774600075186027, 0.9941483444545579, 0.9786193515394289, 1.1468016692540275, 0.0, 0.0], "comment-projection": [[-0.36896427590149633, 1.833324067324042, 0.0, -0.7404200602442671, -0.8928246472656645, -0.861556188402314, 0.9135186166889364, 0.0, 0.0], [-1.460696097065356, 0.6803691646603647, 0.0, 1.392895620536035, -0.43725859627533925, 0.4641301212301663, -0.693280466742397, 0.0, 0.0]]}, "zid": 3, "tids": [0, 1, 2, 3, 4, 5, 6, 7, 8], "mod-in": [0, 1], "n-cmts": 9, "in-conv": [0, 7, 1, 4, 6, 3, 2, 5], "mod-out": [], "repness": {"0": [{"tid": 0, "p-test": 2.0, "n-agree": 3, "repness": 2.4, "n-trials": 3, "n-success": 3, "p-success": 0.8, "best-agree": true, "repful-for": "agree", "repness-test": 1.8973666}, {"tid": 3, "p-test": 1.7320508075688772, "repness": 3.75, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "disagree", "repness-test": 1.9843135}, {"tid": 5, "p-test": 1.4142135623730951, "repness": 2.666666666666667, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "repful-for": "disagree", "repness-test": 1.4907119}], "1": [{"tid": 3, "p-test": 2.0, "n-agree": 3, "repness": 3.2, "n-trials": 3, "n-success": 3, "p-success": 0.8, "best-agree": true, "repful-for": "agree", "repness-test": 1.9321836}, {"tid": 5, "p-test": 1.7320508075688772, "repness": 2.25, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "agree", "repness-test": 1.3693064}]}, "consensus": {"agree": [{"tid": 2, "p-test": 2.449489742783178, "n-trials": 5, "n-success": 5, "p-success": 0.8571428571428571}, {"tid": 7, "p-test": 1.4142135623730951, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667}, {"tid": 8, "p-test": 1.4142135623730951, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667}], "disagree": []}, "meta-tids": [], "votes-base": {"0": {"A": [1, 0, 0, 0, 1, 1, 1, 0], "D": [0, 0, 1, 1, 0, 0, 0, 0], "S": [1, 1, 1, 1, 1, 1, 1, 0]}, "1": {"A": [1, 0, 1, 1, 0, 0, 1, 0], "D": [0, 0, 0, 0, 1, 1, 0, 1], "S": [1, 0, 1, 1, 1, 1, 1, 1]}, "2": {"A": [0, 0, 1, 1, 1, 1, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 1, 1, 1, 1, 1, 0]}, "3": {"A": [0, 0, 0, 1, 0, 1, 0, 1], "D": [0, 0, 0, 0, 1, 0, 1, 0], "S": [0, 0, 0, 1, 1, 1, 1, 1]}, "4": {"A": [0, 0, 0, 0, 1, 1, 0, 0], "D": [0, 0, 0, 0, 0, 0, 1, 1], "S": [0, 0, 0, 0, 1, 1, 1, 1]}, "5": {"A": [0, 0, 0, 0, 0, 1, 0, 1], "D": [0, 0, 0, 0, 0, 0, 1, 0], "S": [0, 0, 0, 0, 0, 1, 1, 1]}, "6": {"A": [0, 0, 0, 0, 0, 0, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0, 1], "S": [0, 0, 0, 0, 0, 0, 1, 1]}, "7": {"A": [0, 0, 0, 0, 0, 0, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 0, 0, 1]}, "8": {"A": [0, 0, 0, 0, 0, 0, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 0, 0, 1]}}, "group-votes": {"0": {"votes": {"0": {"A": 3, "D": 0, "S": 3}, "1": {"A": 2, "D": 1, "S": 3}, "2": {"A": 2, "D": 0, "S": 2}, "3": {"A": 0, "D": 2, "S": 2}, "4": {"A": 1, "D": 1, "S": 2}, "5": {"A": 0, "D": 1, "S": 1}, "6": {"A": 1, "D": 0, "S": 1}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}}, "n-members": 3}, "1": {"votes": {"0": {"A": 1, "D": 2, "S": 4}, "1": {"A": 2, "D": 2, "S": 4}, "2": {"A": 3, "D": 0, "S": 3}, "3": {"A": 3, "D": 0, "S": 3}, "4": {"A": 1, "D": 1, "S": 2}, "5": {"A": 2, "D": 0, "S": 2}, "6": {"A": 0, "D": 1, "S": 1}, "7": {"A": 1, "D": 0, "S": 1}, "8": {"A": 1, "D": 0, "S": 1}}, "n-members": 5}}, "base-clusters": {"x": [1.0354587386118004, 0.14758571036059853, 1.4419090670049066, 0.878519851851234, -1.1607835489278606, -2.1670624574846395, 2.3072635349434583, -1.5372201804555594], "y": [-0.551774465446066, 0.5842784388261424, 1.9108112330858364, 2.35125887995702, -2.1888677176224296, -0.38705523898665917, -1.5322500112473592, 0.7863195266576322], "id": [0, 1, 2, 3, 4, 5, 6, 7], "count": [1, 1, 1, 1, 1, 1, 1, 1], "members": [[1], [0], [2], [3], [4], [5], [6], [7]]}, "group-clusters": [{"id": 0, "center": [0.7273129082091325, -1.4242973981052849], "members": [0, 4, 6]}, {"id": 1, "center": [-0.24725360174469194, 1.0491225679079945], "members": [1, 2, 3, 5, 7]}], "subgroup-votes": {"0": {"0": {"votes": {"0": {"A": 2, "D": 0, "S": 2}, "1": {"A": 2, "D": 0, "S": 2}, "2": {"A": 1, "D": 0, "S": 1}, "3": {"A": 0, "D": 1, "S": 1}, "4": {"A": 0, "D": 1, "S": 1}, "5": {"A": 0, "D": 1, "S": 1}, "6": {"A": 1, "D": 0, "S": 1}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}}, "n-members": 2}, "1": {"votes": {"0": {"A": 1, "D": 0, "S": 1}, "1": {"A": 0, "D": 1, "S": 1}, "2": {"A": 1, "D": 0, "S": 1}, "3": {"A": 0, "D": 1, "S": 1}, "4": {"A": 1, "D": 0, "S": 1}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 0, "D": 0, "S": 0}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}}, "n-members": 1}}, "1": {"0": {"votes": {"0": {"A": 1, "D": 0, "S": 2}, "1": {"A": 0, "D": 2, "S": 2}, "2": {"A": 1, "D": 0, "S": 1}, "3": {"A": 2, "D": 0, "S": 2}, "4": {"A": 1, "D": 1, "S": 2}, "5": {"A": 2, "D": 0, "S": 2}, "6": {"A": 0, "D": 1, "S": 1}, "7": {"A": 1, "D": 0, "S": 1}, "8": {"A": 1, "D": 0, "S": 1}}, "n-members": 3}, "1": {"votes": {"0": {"A": 0, "D": 2, "S": 2}, "1": {"A": 2, "D": 0, "S": 2}, "2": {"A": 2, "D": 0, "S": 2}, "3": {"A": 1, "D": 0, "S": 1}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 0, "D": 0, "S": 0}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}}, "n-members": 2}}}, "lastModTimestamp": null, "subgroup-repness": {"0": {"0": [{"tid": 1, "p-test": 1.7320508075688772, "n-agree": 2, "repness": 2.25, "n-trials": 2, "n-success": 2, "p-success": 0.75, "best-agree": true, "repful-for": "agree", "repness-test": 1.3693064}], "1": [{"tid": 0, "p-test": 0.0, "n-agree": 1, "repness": 1.333333333333333, "n-trials": 1, "n-success": 0, "p-success": 0.3333333333333333, "best-agree": true, "repful-for": "disagree", "repness-test": 0.37267798}, {"tid": 1, "p-test": 1.4142135623730951, "repness": 2.666666666666667, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "repful-for": "disagree", "repness-test": 1.4907119}]}, "1": {"0": [{"tid": 3, "p-test": 1.7320508075688772, "n-agree": 2, "repness": 1.125, "n-trials": 2, "n-success": 2, "p-success": 0.75, "best-agree": true, "repful-for": "agree", "repness-test": 0.0}, {"tid": 1, "p-test": 1.7320508075688772, "repness": 3, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "disagree", "repness-test": 1.7320508}], "1": [{"tid": 1, "p-test": 1.7320508075688772, "n-agree": 2, "repness": 3, "n-trials": 2, "n-success": 2, "p-success": 0.75, "best-agree": true, "repful-for": "agree", "repness-test": 1.7320508}, {"tid": 0, "p-test": 1.7320508075688772, "repness": 3, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "disagree", "repness-test": 1.7320508}]}}, "user-vote-counts": {"0": 1, "1": 2, "2": 3, "3": 4, "4": 5, "5": 6, "6": 7, "7": 7}, "lastVoteTimestamp": 1737731284723, "subgroup-clusters": {"0": [{"id": 0, "center": [1.6713611367776293, -1.0420122383467125], "members": [0, 6], "parent-id": 0}, {"id": 1, "center": [-1.1607835489278606, -2.1888677176224296], "members": [4], "parent-id": 0}], "1": [{"id": 0, "center": [-1.1855656425265333, 0.3278475754990384], "members": [1, 5, 7], "parent-id": 1}, {"id": 1, "center": [1.1602144594280703, 2.131035056521428], "members": [2, 3], "parent-id": 1}]}, "comment-priorities": {"0": 19.065417587230563, "1": 34.620291893361944, "2": 13.494377342773841, "3": 39.843199209142966, "4": 21.610287864439083, "5": 35.551176709944734, "6": 32.33019694206962, "7": 12.529726250088048, "8": 12.529726250088048}, "group-aware-consensus": {"0": 0.26666666666666666, "1": 0.30000000000000004, "2": 0.6000000000000001, "3": 0.2, "4": 0.25, "5": 0.25, "6": 0.2222222222222222, "7": 0.3333333333333333, "8": 0.3333333333333333}}	1737731284723	4838	485	1738590028709
9	prod	{"n": 15, "pca": {"comps": [[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.20881213711437996, -0.0021271035976710853, 0.17043768800918072, -0.43066087849492324, -0.43066087849492324, -0.43066087849492324, -0.43066087849492324, -0.43066087849492324, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.20517118691943337, 0.9702080818117497, -0.11054009834678227, -0.029603880498068794, -0.029603880498068794, -0.029603880498068794, -0.029603880498068794, -0.029603880498068794, 0.0, 0.0]], "center": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.16666666666666669, -0.14285714285714288, -0.4, -0.6, -0.6, -0.6, -0.6, -0.6, -1.0, -1.0], "comment-extremity": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5234763010454684, 5.19339607775557, 0.7611861720127954, 1.0783292334537504, 1.0783292334537504, 1.0783292334537504, 1.0783292334537504, 1.0783292334537504, 0.0, 0.0], "comment-projection": [[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0866928152671622, 0.011386078037007393, -0.6386298122813905, 1.0757905296757166, 1.0757905296757166, 1.0757905296757166, 1.0757905296757166, 1.0757905296757166, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0677447097007375, -5.193383596237994, 0.4141936157510504, 0.073950469782106, 0.073950469782106, 0.073950469782106, 0.073950469782106, 0.073950469782106, 0.0, 0.0]]}, "zid": 9, "tids": [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38], "mod-in": [], "n-cmts": 39, "in-conv": [0, 7, 1, 4, 13, 6, 3, 12, 2, 11, 9, 5, 14, 10, 8], "mod-out": [0, 7, 20, 27, 1, 24, 4, 15, 21, 13, 22, 6, 28, 25, 17, 3, 12, 2, 23, 19, 11, 9, 5, 14, 26, 16, 10, 18, 8], "repness": {"0": [{"tid": 37, "p-test": 2.449489742783178, "n-agree": 5, "repness": 1.714285714285714, "n-trials": 5, "n-success": 5, "p-success": 0.8571428571428571, "best-agree": true, "repful-for": "agree", "repness-test": 0.0}], "1": [{"tid": 30, "p-test": 1.4142135623730951, "n-agree": 1, "repness": 5.333333333333333, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "best-agree": true, "repful-for": "agree", "repness-test": 2.2677867}]}, "consensus": {"agree": [{"tid": 37, "p-test": 2.449489742783178, "n-trials": 5, "n-success": 5, "p-success": 0.8571428571428571}, {"tid": 38, "p-test": 2.23606797749979, "n-trials": 4, "n-success": 4, "p-success": 0.8333333333333333}, {"tid": 32, "p-test": 1.6329931618554516, "n-trials": 5, "n-success": 4, "p-success": 0.7142857142857143}, {"tid": 33, "p-test": 1.6329931618554516, "n-trials": 5, "n-success": 4, "p-success": 0.7142857142857143}, {"tid": 34, "p-test": 1.6329931618554516, "n-trials": 5, "n-success": 4, "p-success": 0.7142857142857143}], "disagree": []}, "meta-tids": [], "votes-base": {"0": {"A": [5, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [5, 0, 0, 0, 0, 0, 0]}, "1": {"A": [5, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [6, 0, 0, 0, 0, 0, 0]}, "2": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "3": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "4": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "5": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "6": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "7": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "8": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [2, 0, 0, 0, 0, 0, 0]}, "9": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "10": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "11": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "12": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "13": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "14": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "15": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "16": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "17": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "18": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "19": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "20": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "21": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "22": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "23": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "24": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "25": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "26": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "27": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "28": {"A": [1, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [1, 0, 0, 0, 0, 0, 0]}, "29": {"A": [0, 0, 1, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [0, 1, 1, 1, 1, 0, 2]}, "30": {"A": [0, 1, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [0, 1, 1, 1, 1, 1, 2]}, "31": {"A": [0, 0, 1, 0, 0, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 1, 0, 1, 1, 2]}, "32": {"A": [0, 0, 0, 0, 1, 1, 2], "D": [0, 0, 1, 0, 0, 0, 0], "S": [0, 0, 1, 0, 1, 1, 2]}, "33": {"A": [0, 0, 0, 0, 1, 1, 2], "D": [0, 0, 1, 0, 0, 0, 0], "S": [0, 0, 1, 0, 1, 1, 2]}, "34": {"A": [0, 0, 0, 0, 1, 1, 2], "D": [0, 0, 1, 0, 0, 0, 0], "S": [0, 0, 1, 0, 1, 1, 2]}, "35": {"A": [0, 0, 0, 0, 1, 1, 2], "D": [0, 0, 1, 0, 0, 0, 0], "S": [0, 0, 1, 0, 1, 1, 2]}, "36": {"A": [0, 0, 0, 0, 1, 1, 2], "D": [0, 0, 1, 0, 0, 0, 0], "S": [0, 0, 1, 0, 1, 1, 2]}, "37": {"A": [0, 0, 1, 0, 1, 1, 2], "D": [0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 1, 0, 1, 1, 2]}, "38": {"A": [0, 0, 1, 0, 0, 1, 2], "D": [0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 1, 0, 0, 1, 2]}}, "group-votes": {"0": {"votes": {"0": {"A": 5, "D": 0, "S": 5}, "1": {"A": 5, "D": 0, "S": 6}, "2": {"A": 1, "D": 0, "S": 1}, "3": {"A": 1, "D": 0, "S": 1}, "4": {"A": 1, "D": 0, "S": 1}, "5": {"A": 1, "D": 0, "S": 1}, "6": {"A": 1, "D": 0, "S": 1}, "7": {"A": 1, "D": 0, "S": 1}, "8": {"A": 1, "D": 0, "S": 2}, "9": {"A": 1, "D": 0, "S": 1}, "10": {"A": 1, "D": 0, "S": 1}, "11": {"A": 1, "D": 0, "S": 1}, "12": {"A": 1, "D": 0, "S": 1}, "13": {"A": 1, "D": 0, "S": 1}, "14": {"A": 1, "D": 0, "S": 1}, "15": {"A": 1, "D": 0, "S": 1}, "16": {"A": 1, "D": 0, "S": 1}, "17": {"A": 1, "D": 0, "S": 1}, "18": {"A": 1, "D": 0, "S": 1}, "19": {"A": 1, "D": 0, "S": 1}, "20": {"A": 1, "D": 0, "S": 1}, "21": {"A": 1, "D": 0, "S": 1}, "22": {"A": 1, "D": 0, "S": 1}, "23": {"A": 1, "D": 0, "S": 1}, "24": {"A": 1, "D": 0, "S": 1}, "25": {"A": 1, "D": 0, "S": 1}, "26": {"A": 1, "D": 0, "S": 1}, "27": {"A": 1, "D": 0, "S": 1}, "28": {"A": 1, "D": 0, "S": 1}, "29": {"A": 1, "D": 0, "S": 5}, "30": {"A": 0, "D": 0, "S": 6}, "31": {"A": 2, "D": 0, "S": 5}, "32": {"A": 4, "D": 1, "S": 5}, "33": {"A": 4, "D": 1, "S": 5}, "34": {"A": 4, "D": 1, "S": 5}, "35": {"A": 4, "D": 1, "S": 5}, "36": {"A": 4, "D": 1, "S": 5}, "37": {"A": 5, "D": 0, "S": 5}, "38": {"A": 4, "D": 0, "S": 4}}, "n-members": 14}, "1": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 0, "D": 0, "S": 0}, "2": {"A": 0, "D": 0, "S": 0}, "3": {"A": 0, "D": 0, "S": 0}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 0, "D": 0, "S": 0}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 0, "D": 0, "S": 0}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 0, "D": 0, "S": 0}, "14": {"A": 0, "D": 0, "S": 0}, "15": {"A": 0, "D": 0, "S": 0}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 0, "D": 0, "S": 0}, "18": {"A": 0, "D": 0, "S": 0}, "19": {"A": 0, "D": 0, "S": 0}, "20": {"A": 0, "D": 0, "S": 0}, "21": {"A": 0, "D": 0, "S": 0}, "22": {"A": 0, "D": 0, "S": 0}, "23": {"A": 0, "D": 0, "S": 0}, "24": {"A": 0, "D": 0, "S": 0}, "25": {"A": 0, "D": 0, "S": 0}, "26": {"A": 0, "D": 0, "S": 0}, "27": {"A": 0, "D": 0, "S": 0}, "28": {"A": 0, "D": 0, "S": 0}, "29": {"A": 0, "D": 0, "S": 1}, "30": {"A": 1, "D": 0, "S": 1}, "31": {"A": 0, "D": 0, "S": 0}, "32": {"A": 0, "D": 0, "S": 0}, "33": {"A": 0, "D": 0, "S": 0}, "34": {"A": 0, "D": 0, "S": 0}, "35": {"A": 0, "D": 0, "S": 0}, "36": {"A": 0, "D": 0, "S": 0}, "37": {"A": 0, "D": 0, "S": 0}, "38": {"A": 0, "D": 0, "S": 0}}, "n-members": 1}}, "base-clusters": {"x": [0.0, 0.04108015824401414, -3.7218636269694025, 0.03869432492848748, 0.976596759569893, 0.7686740627969151, 0.9639949831033912], "y": [0.0, -0.9711137515578815, 0.1390701586675826, 0.11710534934254548, 0.12095849859373899, 0.2675858397202811, 0.1193976783820548], "id": [0, 1, 2, 3, 4, 6, 7], "count": [8, 1, 1, 1, 1, 1, 2], "members": [[0, 2, 3, 4, 5, 1, 7, 6], [9], [8], [10], [11], [13], [12, 14]]}, "group-clusters": [{"id": 0, "center": [-0.0007077509619517475, 0.06310822879201841], "members": [0, 2, 3, 4, 6, 7]}, {"id": 1, "center": [0.04108015824401414, -0.9711137515578815], "members": [1]}], "subgroup-votes": {"0": {"0": {"votes": {"0": {"A": 5, "D": 0, "S": 5}, "1": {"A": 5, "D": 0, "S": 6}, "2": {"A": 1, "D": 0, "S": 1}, "3": {"A": 1, "D": 0, "S": 1}, "4": {"A": 1, "D": 0, "S": 1}, "5": {"A": 1, "D": 0, "S": 1}, "6": {"A": 1, "D": 0, "S": 1}, "7": {"A": 1, "D": 0, "S": 1}, "8": {"A": 1, "D": 0, "S": 2}, "9": {"A": 1, "D": 0, "S": 1}, "10": {"A": 1, "D": 0, "S": 1}, "11": {"A": 1, "D": 0, "S": 1}, "12": {"A": 1, "D": 0, "S": 1}, "13": {"A": 1, "D": 0, "S": 1}, "14": {"A": 1, "D": 0, "S": 1}, "15": {"A": 1, "D": 0, "S": 1}, "16": {"A": 1, "D": 0, "S": 1}, "17": {"A": 1, "D": 0, "S": 1}, "18": {"A": 1, "D": 0, "S": 1}, "19": {"A": 1, "D": 0, "S": 1}, "20": {"A": 1, "D": 0, "S": 1}, "21": {"A": 1, "D": 0, "S": 1}, "22": {"A": 1, "D": 0, "S": 1}, "23": {"A": 1, "D": 0, "S": 1}, "24": {"A": 1, "D": 0, "S": 1}, "25": {"A": 1, "D": 0, "S": 1}, "26": {"A": 1, "D": 0, "S": 1}, "27": {"A": 1, "D": 0, "S": 1}, "28": {"A": 1, "D": 0, "S": 1}, "29": {"A": 0, "D": 0, "S": 4}, "30": {"A": 0, "D": 0, "S": 5}, "31": {"A": 1, "D": 0, "S": 4}, "32": {"A": 4, "D": 0, "S": 4}, "33": {"A": 4, "D": 0, "S": 4}, "34": {"A": 4, "D": 0, "S": 4}, "35": {"A": 4, "D": 0, "S": 4}, "36": {"A": 4, "D": 0, "S": 4}, "37": {"A": 4, "D": 0, "S": 4}, "38": {"A": 3, "D": 0, "S": 3}}, "n-members": 13}, "1": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 0, "D": 0, "S": 0}, "2": {"A": 0, "D": 0, "S": 0}, "3": {"A": 0, "D": 0, "S": 0}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 0, "D": 0, "S": 0}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 0, "D": 0, "S": 0}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 0, "D": 0, "S": 0}, "14": {"A": 0, "D": 0, "S": 0}, "15": {"A": 0, "D": 0, "S": 0}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 0, "D": 0, "S": 0}, "18": {"A": 0, "D": 0, "S": 0}, "19": {"A": 0, "D": 0, "S": 0}, "20": {"A": 0, "D": 0, "S": 0}, "21": {"A": 0, "D": 0, "S": 0}, "22": {"A": 0, "D": 0, "S": 0}, "23": {"A": 0, "D": 0, "S": 0}, "24": {"A": 0, "D": 0, "S": 0}, "25": {"A": 0, "D": 0, "S": 0}, "26": {"A": 0, "D": 0, "S": 0}, "27": {"A": 0, "D": 0, "S": 0}, "28": {"A": 0, "D": 0, "S": 0}, "29": {"A": 1, "D": 0, "S": 1}, "30": {"A": 0, "D": 0, "S": 1}, "31": {"A": 1, "D": 0, "S": 1}, "32": {"A": 0, "D": 1, "S": 1}, "33": {"A": 0, "D": 1, "S": 1}, "34": {"A": 0, "D": 1, "S": 1}, "35": {"A": 0, "D": 1, "S": 1}, "36": {"A": 0, "D": 1, "S": 1}, "37": {"A": 1, "D": 0, "S": 1}, "38": {"A": 1, "D": 0, "S": 1}}, "n-members": 1}}, "1": {"0": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 0, "D": 0, "S": 0}, "2": {"A": 0, "D": 0, "S": 0}, "3": {"A": 0, "D": 0, "S": 0}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 0, "D": 0, "S": 0}, "6": {"A": 0, "D": 0, "S": 0}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 0, "D": 0, "S": 0}, "10": {"A": 0, "D": 0, "S": 0}, "11": {"A": 0, "D": 0, "S": 0}, "12": {"A": 0, "D": 0, "S": 0}, "13": {"A": 0, "D": 0, "S": 0}, "14": {"A": 0, "D": 0, "S": 0}, "15": {"A": 0, "D": 0, "S": 0}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 0, "D": 0, "S": 0}, "18": {"A": 0, "D": 0, "S": 0}, "19": {"A": 0, "D": 0, "S": 0}, "20": {"A": 0, "D": 0, "S": 0}, "21": {"A": 0, "D": 0, "S": 0}, "22": {"A": 0, "D": 0, "S": 0}, "23": {"A": 0, "D": 0, "S": 0}, "24": {"A": 0, "D": 0, "S": 0}, "25": {"A": 0, "D": 0, "S": 0}, "26": {"A": 0, "D": 0, "S": 0}, "27": {"A": 0, "D": 0, "S": 0}, "28": {"A": 0, "D": 0, "S": 0}, "29": {"A": 0, "D": 0, "S": 1}, "30": {"A": 1, "D": 0, "S": 1}, "31": {"A": 0, "D": 0, "S": 0}, "32": {"A": 0, "D": 0, "S": 0}, "33": {"A": 0, "D": 0, "S": 0}, "34": {"A": 0, "D": 0, "S": 0}, "35": {"A": 0, "D": 0, "S": 0}, "36": {"A": 0, "D": 0, "S": 0}, "37": {"A": 0, "D": 0, "S": 0}, "38": {"A": 0, "D": 0, "S": 0}}, "n-members": 1}}}, "lastModTimestamp": null, "subgroup-repness": {"0": {"0": [{"tid": 32, "p-test": 2.23606797749979, "n-agree": 4, "repness": 2.5, "n-trials": 4, "n-success": 4, "p-success": 0.8333333333333333, "best-agree": true, "repful-for": "agree", "repness-test": 1.7078252}, {"tid": 33, "p-test": 2.23606797749979, "repness": 2.5, "n-trials": 4, "n-success": 4, "p-success": 0.8333333333333333, "repful-for": "agree", "repness-test": 1.7078252}, {"tid": 34, "p-test": 2.23606797749979, "repness": 2.5, "n-trials": 4, "n-success": 4, "p-success": 0.8333333333333333, "repful-for": "agree", "repness-test": 1.7078252}, {"tid": 35, "p-test": 2.23606797749979, "repness": 2.5, "n-trials": 4, "n-success": 4, "p-success": 0.8333333333333333, "repful-for": "agree", "repness-test": 1.7078252}, {"tid": 36, "p-test": 2.23606797749979, "repness": 2.5, "n-trials": 4, "n-success": 4, "p-success": 0.8333333333333333, "repful-for": "agree", "repness-test": 1.7078252}], "1": [{"tid": 29, "p-test": 1.4142135623730951, "n-agree": 1, "repness": 4, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "best-agree": true, "repful-for": "agree", "repness-test": 1.9321836}, {"tid": 32, "p-test": 1.4142135623730951, "repness": 4, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "repful-for": "disagree", "repness-test": 1.9321836}, {"tid": 33, "p-test": 1.4142135623730951, "repness": 4, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "repful-for": "disagree", "repness-test": 1.9321836}, {"tid": 34, "p-test": 1.4142135623730951, "repness": 4, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "repful-for": "disagree", "repness-test": 1.9321836}, {"tid": 35, "p-test": 1.4142135623730951, "repness": 4, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "repful-for": "disagree", "repness-test": 1.9321836}]}, "1": {"0": [{"tid": 30, "p-test": 1.4142135623730951, "n-agree": 1, "repness": 1.333333333333333, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "best-agree": true, "repful-for": "agree", "repness-test": 0.0}]}}, "user-vote-counts": {"0": 1, "1": 1, "2": 2, "3": 2, "4": 3, "5": 19, "6": 10, "7": 1, "8": 10, "9": 2, "10": 2, "11": 9, "12": 10, "13": 9, "14": 10}, "lastVoteTimestamp": 1739462776620, "subgroup-clusters": {"0": [{"id": 0, "center": [0.2855350087309291, 0.057265003416975005], "members": [0, 3, 4, 6, 7], "parent-id": 0}, {"id": 1, "center": [-3.7218636269694025, 0.1390701586675826], "members": [2], "parent-id": 0}], "1": [{"id": 0, "center": [0.04108015824401414, -0.9711137515578815], "members": [1], "parent-id": 1}]}, "comment-priorities": {"0": 13.494377342773841, "1": 6.356644922322759, "2": 12.529726250088048, "3": 12.529726250088048, "4": 12.529726250088048, "5": 12.529726250088048, "6": 12.529726250088048, "7": 12.529726250088048, "8": 3.1177549932492687, "9": 12.529726250088048, "10": 12.529726250088048, "11": 12.529726250088048, "12": 12.529726250088048, "13": 12.529726250088048, "14": 12.529726250088048, "15": 12.529726250088048, "16": 12.529726250088048, "17": 12.529726250088048, "18": 12.529726250088048, "19": 12.529726250088048, "20": 12.529726250088048, "21": 12.529726250088048, "22": 12.529726250088048, "23": 12.529726250088048, "24": 12.529726250088048, "25": 12.529726250088048, "26": 12.529726250088048, "27": 12.529726250088048, "28": 12.529726250088048, "29": 0.49973687276628764, "30": 1.52029048006127, "31": 2.6160341038277375, "32": 40.47800044112462, "33": 40.47800044112462, "34": 40.47800044112462, "35": 40.47800044112462, "36": 40.47800044112462, "37": 13.494377342773841, "38": 15.095347903328785}, "group-aware-consensus": {"0": 0.42857142857142855, "1": 0.375, "2": 0.3333333333333333, "3": 0.3333333333333333, "4": 0.3333333333333333, "5": 0.3333333333333333, "6": 0.3333333333333333, "7": 0.3333333333333333, "8": 0.25, "9": 0.3333333333333333, "10": 0.3333333333333333, "11": 0.3333333333333333, "12": 0.3333333333333333, "13": 0.3333333333333333, "14": 0.3333333333333333, "15": 0.3333333333333333, "16": 0.3333333333333333, "17": 0.3333333333333333, "18": 0.3333333333333333, "19": 0.3333333333333333, "20": 0.3333333333333333, "21": 0.3333333333333333, "22": 0.3333333333333333, "23": 0.3333333333333333, "24": 0.3333333333333333, "25": 0.3333333333333333, "26": 0.3333333333333333, "27": 0.3333333333333333, "28": 0.3333333333333333, "29": 0.09523809523809523, "30": 0.08333333333333333, "31": 0.21428571428571427, "32": 0.3571428571428571, "33": 0.3571428571428571, "34": 0.3571428571428571, "35": 0.3571428571428571, "36": 0.3571428571428571, "37": 0.42857142857142855, "38": 0.41666666666666663}}	1739462776620	6475	259	1739777950519
2	prod	{"n": 93, "pca": {"comps": [[0.0, 0.0, 0.0, -0.01371480830002387, 0.0, -0.17279592715912978, 0.0, 0.0, 0.0, 0.057475002515020926, 0.004464085635678495, 0.12348204744346238, 0.09903645969050243, 0.19426140936713304, 0.0, 0.021344536905147555, 0.0, 0.0037608981651293103, 0.0, 0.0, -0.15333654502364943, 0.040880466535807604, 0.2675566020411333, -0.13783736999066357, 0.07195687753012085, 0.04319039114589815, 0.0, -0.20815414693422873, 0.08916326772285194, -0.019486268181980154, 0.4026376576757669, 0.10568967481276709, -0.028059496653846702, 0.33538239196867287, 0.24237715558199519, 0.1900740018640635, -0.012685059657103443, -0.09506036591377846, 0.19484920275609852, 0.13873991251812973, 0.05815742595428595, -0.19211442326484143, 0.0053570467763185345, 0.1365228405615995, 0.0320824369037674, -0.018903844295176636, 0.09723443884987479, -0.010460821831893863, -0.21948850952900364, 0.06033310973223006, 0.06841528291287127, -0.03960003510374316, 0.04483143026461085, 0.07652865024337217, 0.023414555540355392, 0.011744572898738835, -0.03717694874456903, -0.036468401661806246, 0.19415561571916035, 0.12586797200379782, -0.011164991234304623, -0.08797199456996191, 0.046172878575307066, 0.00895909840722381, -0.0837051773445838, -0.004166642328401923, 0.03159817081152824, 0.028640161735371723, 0.02016948692437722, 0.16590823550674508, 0.18895927071005722, 0.03477405203786712, -0.06686598538114971, -0.006379760647467927], [0.0, 0.0, 0.0, -0.10310000757949087, 0.0, -0.24365534837006256, 0.0, 0.0, 0.0, -0.08903819893835707, -0.02224580708555968, -0.014939828316119057, -0.20109933517352777, -0.23097549843296858, 0.0, -0.048809888590516264, 0.0, -0.037821462000577996, 0.0, 0.0, -0.24690853772701893, -0.010112435242043132, 0.07277269380579107, -0.23169074855948932, 0.08832705979553879, -0.10493996916902952, 0.0, -0.28342169993644434, -0.165982071191351, -0.13955047410715699, 0.0555416000986771, -0.12366972825061281, -0.10107924105610014, -0.13346187701968876, -0.2180927014064516, -0.11762943839795505, -0.016391590934651333, -0.12301278956011788, -0.38667815637948943, -0.010981652624817354, -0.16026355971803855, -0.07673807771390538, 0.0030631604217410424, -0.10962133837189547, -0.048286413376122314, 0.017229084873807872, -0.026017454540926407, -0.10348325231777923, -0.06237886164628156, 0.08895571165106665, 0.0674471979806034, -0.29945178545208073, 0.04298066225379107, -0.048858809116675506, -0.021548253717346543, -0.00229701065613762, -0.10902002009117752, -0.18964752758820094, 0.1874939232058416, 0.05859551107263395, 0.0022864986206204076, -0.016657892098026834, 0.07281955940818467, -0.02885525380300295, -0.04584322941184513, 0.03394362630814019, 0.03269226777943228, -0.08801788223361412, 0.02488773888978902, -0.015323511762394482, -0.07350097174664792, 0.0546328782299308, 0.044530147155133135, 0.014150235535780846]], "center": [0.0, 0.0, 0.0, -0.9230769230769225, 0.0, -0.7142857142857144, 0.0, 0.0, 0.0, -0.8813559322033901, -0.9344262295081962, -0.4210526315789474, -0.7999999999999997, -0.7656250000000001, 0.0, -0.9661016949152537, 0.0, -0.9677419354838713, 0.0, 0.0, -0.6774193548387101, -0.9354838709677423, -0.08771929824561399, -0.6833333333333327, 0.10714285714285712, -0.9032258064516135, 0.0, -0.596491228070176, -0.7636363636363641, -0.6181818181818179, -0.07894736842105261, -0.851851851851852, -0.686274509803922, -0.5833333333333337, -0.7333333333333336, -0.8076923076923074, -0.8039215686274506, -0.7037037037037036, -0.4642857142857144, -0.6666666666666663, -0.847826086956522, -0.1538461538461538, -0.978260869565218, -0.6923076923076927, -0.7674418604651166, -0.7692307692307694, -0.8863636363636365, -0.6666666666666662, -0.06976744186046516, -0.21951219512195108, -0.5263157894736845, -0.17073170731707318, -0.3000000000000004, -0.782608695652174, -0.9512195121951228, -0.7906976744186051, -0.3720930232558143, -0.16666666666666657, -0.20588235294117657, -0.588235294117647, -0.774193548387097, -0.32258064516129054, -0.2800000000000002, -0.9090909090909084, -0.7096774193548391, -0.6000000000000008, -0.8666666666666663, -0.5769230769230775, -0.7419354838709675, -0.4642857142857146, -0.4642857142857146, -0.19047619047619027, -0.4583333333333334, -0.625], "comment-extremity": [0.0, 0.0, 0.0, 0.0688240341427456, 0.0, 0.7341662343692381, 0.0, 0.0, 0.0, 0.10816195287328079, 0.012798732723369483, 0.6194615299326315, 0.38566503905277705, 0.6084932485134069, 0.0, 0.015534583616925649, 0.0, 0.010547003133653731, 0.0, 0.0, 0.8065302046304085, 0.023372036789594687, 2.1759946065305282, 0.7343868698836981, 1.0850452828838735, 0.09447053548007635, 0.0, 1.220608676558015, 0.3830994537457797, 0.4628039052253828, 3.220385746891548, 0.20732134031835125, 0.28310518982773836, 1.2937961738981332, 0.7479523352444386, 0.36978104084232966, 0.034960311104216206, 0.39624884148286615, 1.995417380785127, 0.3990729035977395, 0.2231792908253512, 1.5058102662752746, 0.0011540154976961384, 0.4634313964319009, 0.11597720704125436, 0.05077476652144667, 0.09839405583390973, 0.29824443935789063, 1.8259373461832193, 0.7216609060094483, 0.39147166773298114, 2.1547775932833173, 0.3739811110015239, 0.16979395075564566, 0.013352858205845634, 0.021546586750029444, 0.6221649737353787, 1.3844156280635354, 1.8438118460503357, 0.49178521539983, 0.022137668574092897, 0.521755947820713, 0.5340449716032722, 0.023628310272550843, 0.2383482473752114, 0.1176743098336914, 0.05214934606063643, 0.3368680966227706, 0.07111509557882534, 0.7678238125251224, 0.9343557344627645, 0.45098176170597304, 0.374336563708522, 0.05007177984377136], "comment-projection": [[0.0, 0.0, 0.0, 0.009075326305534016, 0.0, 0.42469907721229677, 0.0, 0.0, 0.0, -0.05865984177132393, -0.00251813223987144, -0.6149768475915569, -0.17038876791081178, -0.39166402270551537, 0.0, -0.00622415759771111, 0.0, -0.0010436280423440567, 0.0, 0.0, 0.42550026955417974, -0.02268819807803008, -2.099713398970711, 0.3754785986942404, -0.6853175156128702, -0.035955270294949486, 0.0, 0.7225267120188841, -0.18129361092212476, 0.06400311927248277, -3.1901764042907583, -0.13469288297887175, 0.07572609162223785, -1.2021118435638776, -0.5560019012311779, -0.3144381517006045, 0.02139627631416554, 0.2422933889168635, -0.8979408322135167, -0.3978286183339974, -0.07613094920149995, 1.3983798716583955, -0.0010018056269707168, -0.36135811719737776, -0.06418222269897178, 0.03752700402107018, -0.09505025797102142, 0.029995797319510617, 1.7563828384622089, -0.40507709993273605, -0.2787776132462819, 0.28249221967453564, -0.2699581817260449, -0.1431139873133105, -0.00982534745079847, -0.02114594711109313, 0.20080980347669247, 0.2614275442200127, -1.3263271614310908, -0.4458412147739876, 0.021687554948650843, 0.5126463853325071, -0.2859797664147276, -0.007006279872580303, 0.20904943414459248, 0.014337125032136445, -0.03624236575524521, -0.10423430217046643, -0.0447753514820274, -0.7645696105232844, -0.8707977368952171, -0.24215909572417105, 0.3115682675895654, 0.02058029108077409], [0.0, 0.0, 0.0, 0.06822306155641136, 0.0, 0.5988578742096747, 0.0, 0.0, 0.0, 0.09087370913925025, 0.012548568418220815, 0.07440473098417517, 0.3459843784297429, 0.4656858670869584, 0.0, 0.014233170776397993, 0.0, 0.010495242522711647, 0.0, 0.0, 0.685157275076671, 0.005612287560900608, -0.5711008403362856, 0.6311417404832177, -0.8412271801973285, 0.08736074983600751, 0.0, 0.9837889467298816, 0.3374875080638275, 0.4583569083315505, -0.4400668906981303, 0.15760699675533943, 0.2727894931169824, 0.4783676986972514, 0.5002944948976799, 0.19459359424591627, 0.02764819546356438, 0.3135395637933036, 1.7819631829482543, 0.0314892492827838, 0.20979293226175502, 0.5585680733452846, -0.000572832658542331, 0.2901533565795645, 0.09659893810239102, -0.03420235203184137, 0.02543302347517545, 0.29673219870966033, 0.49916582108759444, -0.597249534426325, -0.27483287465357564, 2.136179913382578, -0.2588135458300084, 0.09136942786565005, 0.009042199386018154, 0.004135737098921603, 0.588867283325196, 1.3595080986701207, -1.2808194191554452, -0.2075531481204568, -0.004441433354856361, 0.09707189444842078, -0.45102062579899566, 0.02256566171604564, 0.11449113987842319, -0.11679764569822666, -0.03749726948745796, 0.32033626827645384, -0.055249658088049464, 0.07061669593578795, 0.3387210355759153, -0.38045173379743563, -0.20749235542626931, -0.04564684825651951]]}, "zid": 2, "tids": [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73], "mod-in": [65, 70, 62, 59, 20, 72, 58, 60, 27, 1, 69, 24, 55, 39, 46, 54, 15, 48, 50, 21, 31, 32, 40, 56, 33, 13, 22, 36, 41, 43, 61, 29, 44, 6, 28, 64, 51, 25, 34, 17, 3, 12, 2, 66, 23, 47, 35, 57, 68, 11, 9, 5, 45, 53, 38, 30, 73, 10, 52, 67, 71, 42, 37, 63, 49], "n-cmts": 74, "in-conv": [0, 65, 70, 62, 74, 7, 59, 86, 20, 72, 58, 60, 27, 1, 69, 24, 55, 85, 39, 88, 4, 77, 92, 15, 50, 75, 21, 31, 32, 91, 56, 33, 13, 22, 90, 36, 41, 89, 43, 61, 29, 44, 6, 28, 64, 51, 25, 17, 3, 12, 2, 66, 23, 47, 82, 76, 19, 57, 68, 11, 9, 5, 14, 45, 53, 78, 26, 16, 81, 79, 38, 87, 30, 73, 10, 18, 52, 67, 71, 63, 8, 49, 84], "mod-out": [0, 7, 1, 4, 6, 2, 19, 14, 26, 16, 18, 8], "repness": {"0": [{"tid": 30, "p-test": 3.528211425363985, "n-agree": 23, "repness": 3.6, "n-trials": 28, "n-success": 23, "p-success": 0.8, "best-agree": true, "repful-for": "agree", "repness-test": 5.0314493}, {"tid": 33, "p-test": 5.0990195135927845, "repness": 1.781481481481481, "n-trials": 25, "n-success": 25, "p-success": 0.962962962962963, "repful-for": "agree", "repness-test": 3.9465022}, {"tid": 22, "p-test": 1.9639610121239315, "repness": 3.798701298701299, "n-trials": 20, "n-success": 14, "p-success": 0.6818181818181818, "repful-for": "agree", "repness-test": 4.0313706}, {"tid": 43, "p-test": 4.123105625617661, "repness": 1.574074074074074, "n-trials": 16, "n-success": 16, "p-success": 0.9444444444444444, "repful-for": "agree", "repness-test": 2.857966}, {"tid": 70, "p-test": 3.3166247903554, "repness": 1.833333333333333, "n-trials": 10, "n-success": 10, "p-success": 0.9166666666666667, "repful-for": "agree", "repness-test": 2.7282977}], "1": [{"tid": 27, "p-test": 4.898979485566356, "n-agree": 23, "repness": 1.818947368421053, "n-trials": 23, "n-success": 23, "p-success": 0.96, "best-agree": true, "repful-for": "agree", "repness-test": 3.8799253}, {"tid": 37, "p-test": 4.2, "repness": 1.489878542510121, "n-trials": 24, "n-success": 22, "p-success": 0.8846153846153846, "repful-for": "agree", "repness-test": 2.6383443}, {"tid": 20, "p-test": 4.270992778072193, "repness": 1.418181818181818, "n-trials": 28, "n-success": 25, "p-success": 0.8666666666666667, "repful-for": "agree", "repness-test": 2.4645932}, {"tid": 3, "p-test": 5.5677643628300215, "repness": 1.125, "n-trials": 30, "n-success": 30, "p-success": 0.96875, "repful-for": "agree", "repness-test": 1.9420166}, {"tid": 40, "p-test": 4.358898943540674, "repness": 1.252272727272727, "n-trials": 18, "n-success": 18, "p-success": 0.95, "repful-for": "agree", "repness-test": 2.1603813}], "2": [{"tid": 3, "p-test": -1.8898223650461359, "n-agree": 6, "repness": 3.75, "n-trials": 6, "n-success": 0, "p-success": 0.125, "best-agree": true, "repful-for": "disagree", "repness-test": 1.3085146}, {"tid": 33, "p-test": 1.8898223650461359, "repness": 8.4, "n-trials": 6, "n-success": 5, "p-success": 0.75, "repful-for": "disagree", "repness-test": 4.998113}, {"tid": 30, "p-test": 2.6457513110645907, "repness": 2.791666666666667, "n-trials": 6, "n-success": 6, "p-success": 0.875, "repful-for": "disagree", "repness-test": 3.5274894}, {"tid": 69, "p-test": 1.7320508075688772, "repness": 4.2, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "disagree", "repness-test": 3.0276504}, {"tid": 70, "p-test": 1.7320508075688772, "repness": 4.2, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "disagree", "repness-test": 3.0276504}], "3": [{"tid": 35, "p-test": 2.23606797749979, "n-agree": 4, "repness": 0.9920634920634921, "n-trials": 4, "n-success": 4, "p-success": 0.8333333333333333, "best-agree": true, "repful-for": "agree", "repness-test": 0.9059077}, {"tid": 62, "p-test": 1.341640786499874, "repness": 2.555555555555556, "n-trials": 4, "n-success": 3, "p-success": 0.6666666666666667, "repful-for": "agree", "repness-test": 2.2038858}, {"tid": 65, "p-test": 2.23606797749979, "repness": 1.277777777777778, "n-trials": 4, "n-success": 4, "p-success": 0.8333333333333333, "repful-for": "agree", "repness-test": 1.4655126}, {"tid": 49, "p-test": 1.341640786499874, "repness": 1.857142857142857, "n-trials": 4, "n-success": 3, "p-success": 0.6666666666666667, "repful-for": "agree", "repness-test": 1.8389293}, {"tid": 69, "p-test": 2.0, "repness": 1.270588235294118, "n-trials": 3, "n-success": 3, "p-success": 0.8, "repful-for": "agree", "repness-test": 1.4064217}]}, "consensus": {"agree": [{"tid": 17, "p-test": 7.433301302514803, "n-trials": 62, "n-success": 60, "p-success": 0.953125}, {"tid": 15, "p-test": 7.229568912920512, "n-trials": 59, "n-success": 57, "p-success": 0.9508196721311475}, {"tid": 21, "p-test": 7.181324987175318, "n-trials": 62, "n-success": 59, "p-success": 0.9375}, {"tid": 3, "p-test": 7.1393064768013, "n-trials": 65, "n-success": 61, "p-success": 0.9253731343283582}, {"tid": 10, "p-test": 6.858006858010286, "n-trials": 61, "n-success": 57, "p-success": 0.9206349206349206}], "disagree": []}, "meta-tids": [1, 6, 2], "votes-base": {"0": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}, "1": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0], "D": [0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1], "S": [1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]}, "2": {"A": [0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0], "D": [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1], "S": [1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]}, "3": {"A": [0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1]}, "4": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}, "5": {"A": [0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0], "S": [0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1]}, "6": {"A": [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0], "D": [0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1], "S": [1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]}, "7": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}, "8": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}, "9": {"A": [0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1]}, "10": {"A": [0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "11": {"A": [0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0], "S": [0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1]}, "12": {"A": [0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "13": {"A": [0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1]}, "14": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}, "15": {"A": [0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1]}, "16": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}, "17": {"A": [0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]}, "18": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}, "19": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}, "20": {"A": [0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0], "S": [0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1]}, "21": {"A": [0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "22": {"A": [0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0], "S": [0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1]}, "23": {"A": [0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0], "S": [1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "24": {"A": [0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0], "D": [0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0], "S": [0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1]}, "25": {"A": [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1]}, "26": {"A": [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}, "27": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "28": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "29": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1]}, "30": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1], "D": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "31": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "32": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1]}, "33": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "34": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "35": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1]}, "36": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1]}, "37": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "38": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "39": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "40": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "41": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1]}, "42": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "43": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1]}, "44": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1]}, "45": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1]}, "46": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "47": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1]}, "48": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1]}, "49": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1]}, "50": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1]}, "51": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "52": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1]}, "53": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1]}, "54": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1]}, "55": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1]}, "56": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "57": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1]}, "58": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1]}, "59": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "60": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1]}, "61": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "62": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1]}, "63": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "64": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "65": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1]}, "66": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "67": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1]}, "68": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1]}, "69": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "70": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "71": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1]}, "72": {"A": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0], "S": [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}, "73": {"A": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1], "D": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "S": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]}}, "group-votes": {"0": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 0, "D": 25, "S": 31}, "2": {"A": 14, "D": 8, "S": 31}, "3": {"A": 22, "D": 0, "S": 23}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 16, "D": 2, "S": 24}, "6": {"A": 8, "D": 20, "S": 31}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 22, "D": 0, "S": 22}, "10": {"A": 21, "D": 0, "S": 22}, "11": {"A": 15, "D": 1, "S": 21}, "12": {"A": 26, "D": 0, "S": 27}, "13": {"A": 24, "D": 0, "S": 25}, "14": {"A": 0, "D": 0, "S": 0}, "15": {"A": 22, "D": 0, "S": 22}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 22, "D": 0, "S": 22}, "18": {"A": 0, "D": 0, "S": 0}, "19": {"A": 0, "D": 0, "S": 0}, "20": {"A": 14, "D": 2, "S": 23}, "21": {"A": 22, "D": 0, "S": 22}, "22": {"A": 14, "D": 0, "S": 20}, "23": {"A": 15, "D": 1, "S": 24}, "24": {"A": 7, "D": 8, "S": 20}, "25": {"A": 23, "D": 0, "S": 23}, "26": {"A": 0, "D": 0, "S": 0}, "27": {"A": 11, "D": 5, "S": 24}, "28": {"A": 18, "D": 0, "S": 21}, "29": {"A": 16, "D": 1, "S": 22}, "30": {"A": 23, "D": 0, "S": 28}, "31": {"A": 22, "D": 0, "S": 22}, "32": {"A": 13, "D": 0, "S": 20}, "33": {"A": 25, "D": 0, "S": 25}, "34": {"A": 22, "D": 1, "S": 23}, "35": {"A": 21, "D": 0, "S": 22}, "36": {"A": 15, "D": 0, "S": 20}, "37": {"A": 11, "D": 2, "S": 20}, "38": {"A": 19, "D": 3, "S": 23}, "39": {"A": 19, "D": 0, "S": 21}, "40": {"A": 19, "D": 0, "S": 20}, "41": {"A": 2, "D": 4, "S": 16}, "42": {"A": 21, "D": 0, "S": 21}, "43": {"A": 16, "D": 0, "S": 16}, "44": {"A": 15, "D": 1, "S": 18}, "45": {"A": 11, "D": 1, "S": 15}, "46": {"A": 20, "D": 0, "S": 20}, "47": {"A": 10, "D": 0, "S": 16}, "48": {"A": 4, "D": 9, "S": 18}, "49": {"A": 8, "D": 2, "S": 18}, "50": {"A": 11, "D": 0, "S": 15}, "51": {"A": 7, "D": 5, "S": 17}, "52": {"A": 10, "D": 3, "S": 19}, "53": {"A": 18, "D": 0, "S": 19}, "54": {"A": 17, "D": 0, "S": 18}, "55": {"A": 16, "D": 1, "S": 18}, "56": {"A": 10, "D": 4, "S": 18}, "57": {"A": 2, "D": 0, "S": 11}, "58": {"A": 10, "D": 1, "S": 13}, "59": {"A": 10, "D": 0, "S": 12}, "60": {"A": 6, "D": 0, "S": 9}, "61": {"A": 3, "D": 3, "S": 11}, "62": {"A": 2, "D": 0, "S": 8}, "63": {"A": 10, "D": 0, "S": 10}, "64": {"A": 7, "D": 2, "S": 10}, "65": {"A": 5, "D": 2, "S": 8}, "66": {"A": 10, "D": 0, "S": 11}, "67": {"A": 9, "D": 0, "S": 9}, "68": {"A": 7, "D": 0, "S": 9}, "69": {"A": 10, "D": 0, "S": 11}, "70": {"A": 10, "D": 0, "S": 10}, "71": {"A": 3, "D": 1, "S": 6}, "72": {"A": 3, "D": 3, "S": 8}, "73": {"A": 3, "D": 0, "S": 5}}, "n-members": 31}, "1": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 1, "D": 30, "S": 40}, "2": {"A": 8, "D": 15, "S": 39}, "3": {"A": 30, "D": 0, "S": 30}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 24, "D": 0, "S": 27}, "6": {"A": 8, "D": 26, "S": 40}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 25, "D": 1, "S": 26}, "10": {"A": 27, "D": 0, "S": 28}, "11": {"A": 11, "D": 3, "S": 25}, "12": {"A": 28, "D": 0, "S": 31}, "13": {"A": 25, "D": 0, "S": 27}, "14": {"A": 0, "D": 0, "S": 0}, "15": {"A": 26, "D": 0, "S": 26}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 29, "D": 0, "S": 29}, "18": {"A": 0, "D": 0, "S": 0}, "19": {"A": 0, "D": 0, "S": 0}, "20": {"A": 25, "D": 0, "S": 28}, "21": {"A": 28, "D": 1, "S": 29}, "22": {"A": 3, "D": 10, "S": 26}, "23": {"A": 21, "D": 0, "S": 25}, "24": {"A": 5, "D": 9, "S": 25}, "25": {"A": 26, "D": 0, "S": 28}, "26": {"A": 1, "D": 0, "S": 1}, "27": {"A": 23, "D": 0, "S": 23}, "28": {"A": 20, "D": 1, "S": 24}, "29": {"A": 16, "D": 1, "S": 23}, "30": {"A": 6, "D": 19, "S": 33}, "31": {"A": 20, "D": 1, "S": 22}, "32": {"A": 15, "D": 0, "S": 21}, "33": {"A": 16, "D": 3, "S": 25}, "34": {"A": 24, "D": 0, "S": 27}, "35": {"A": 18, "D": 0, "S": 20}, "36": {"A": 19, "D": 1, "S": 21}, "37": {"A": 22, "D": 0, "S": 24}, "38": {"A": 17, "D": 2, "S": 22}, "39": {"A": 14, "D": 3, "S": 20}, "40": {"A": 18, "D": 0, "S": 18}, "41": {"A": 9, "D": 2, "S": 16}, "42": {"A": 17, "D": 0, "S": 18}, "43": {"A": 12, "D": 1, "S": 16}, "44": {"A": 16, "D": 0, "S": 18}, "45": {"A": 15, "D": 1, "S": 17}, "46": {"A": 15, "D": 0, "S": 17}, "47": {"A": 14, "D": 1, "S": 16}, "48": {"A": 7, "D": 2, "S": 17}, "49": {"A": 5, "D": 5, "S": 16}, "50": {"A": 6, "D": 1, "S": 16}, "51": {"A": 8, "D": 2, "S": 17}, "52": {"A": 5, "D": 4, "S": 14}, "53": {"A": 15, "D": 2, "S": 19}, "54": {"A": 16, "D": 0, "S": 16}, "55": {"A": 14, "D": 1, "S": 18}, "56": {"A": 11, "D": 3, "S": 17}, "57": {"A": 5, "D": 0, "S": 12}, "58": {"A": 3, "D": 6, "S": 14}, "59": {"A": 10, "D": 2, "S": 15}, "60": {"A": 12, "D": 1, "S": 15}, "61": {"A": 8, "D": 2, "S": 13}, "62": {"A": 3, "D": 1, "S": 11}, "63": {"A": 15, "D": 0, "S": 17}, "64": {"A": 13, "D": 0, "S": 15}, "65": {"A": 8, "D": 1, "S": 11}, "66": {"A": 11, "D": 0, "S": 13}, "67": {"A": 5, "D": 1, "S": 11}, "68": {"A": 11, "D": 1, "S": 16}, "69": {"A": 6, "D": 4, "S": 12}, "70": {"A": 7, "D": 3, "S": 13}, "71": {"A": 3, "D": 2, "S": 11}, "72": {"A": 7, "D": 0, "S": 12}, "73": {"A": 4, "D": 0, "S": 8}}, "n-members": 41}, "2": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 0, "D": 3, "S": 6}, "2": {"A": 1, "D": 3, "S": 6}, "3": {"A": 6, "D": 0, "S": 6}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 6, "D": 0, "S": 6}, "6": {"A": 1, "D": 3, "S": 6}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 3, "D": 0, "S": 6}, "10": {"A": 5, "D": 0, "S": 6}, "11": {"A": 1, "D": 1, "S": 6}, "12": {"A": 2, "D": 1, "S": 6}, "13": {"A": 1, "D": 2, "S": 6}, "14": {"A": 0, "D": 0, "S": 0}, "15": {"A": 5, "D": 0, "S": 6}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 6, "D": 0, "S": 6}, "18": {"A": 0, "D": 0, "S": 0}, "19": {"A": 0, "D": 0, "S": 0}, "20": {"A": 6, "D": 0, "S": 6}, "21": {"A": 5, "D": 0, "S": 6}, "22": {"A": 0, "D": 4, "S": 6}, "23": {"A": 6, "D": 0, "S": 6}, "24": {"A": 0, "D": 3, "S": 6}, "25": {"A": 4, "D": 0, "S": 6}, "26": {"A": 0, "D": 0, "S": 0}, "27": {"A": 6, "D": 0, "S": 6}, "28": {"A": 3, "D": 1, "S": 6}, "29": {"A": 5, "D": 1, "S": 6}, "30": {"A": 0, "D": 6, "S": 6}, "31": {"A": 3, "D": 1, "S": 6}, "32": {"A": 5, "D": 1, "S": 6}, "33": {"A": 0, "D": 5, "S": 6}, "34": {"A": 0, "D": 4, "S": 6}, "35": {"A": 2, "D": 3, "S": 6}, "36": {"A": 5, "D": 0, "S": 6}, "37": {"A": 5, "D": 0, "S": 6}, "38": {"A": 0, "D": 4, "S": 6}, "39": {"A": 2, "D": 2, "S": 5}, "40": {"A": 1, "D": 0, "S": 3}, "41": {"A": 3, "D": 0, "S": 3}, "42": {"A": 3, "D": 0, "S": 3}, "43": {"A": 0, "D": 1, "S": 3}, "44": {"A": 1, "D": 0, "S": 3}, "45": {"A": 3, "D": 0, "S": 3}, "46": {"A": 1, "D": 1, "S": 3}, "47": {"A": 2, "D": 0, "S": 3}, "48": {"A": 3, "D": 0, "S": 3}, "49": {"A": 0, "D": 0, "S": 3}, "50": {"A": 1, "D": 0, "S": 3}, "51": {"A": 1, "D": 1, "S": 3}, "52": {"A": 1, "D": 0, "S": 3}, "53": {"A": 2, "D": 1, "S": 3}, "54": {"A": 2, "D": 0, "S": 3}, "55": {"A": 3, "D": 0, "S": 3}, "56": {"A": 1, "D": 0, "S": 3}, "57": {"A": 0, "D": 0, "S": 3}, "58": {"A": 0, "D": 2, "S": 3}, "59": {"A": 0, "D": 1, "S": 3}, "60": {"A": 3, "D": 0, "S": 3}, "61": {"A": 3, "D": 0, "S": 3}, "62": {"A": 0, "D": 0, "S": 2}, "63": {"A": 2, "D": 0, "S": 2}, "64": {"A": 2, "D": 0, "S": 2}, "65": {"A": 1, "D": 0, "S": 2}, "66": {"A": 1, "D": 0, "S": 2}, "67": {"A": 2, "D": 0, "S": 2}, "68": {"A": 2, "D": 0, "S": 2}, "69": {"A": 0, "D": 2, "S": 2}, "70": {"A": 0, "D": 2, "S": 2}, "71": {"A": 0, "D": 1, "S": 1}, "72": {"A": 2, "D": 0, "S": 2}, "73": {"A": 2, "D": 0, "S": 2}}, "n-members": 6}, "3": {"votes": {"0": {"A": 0, "D": 0, "S": 1}, "1": {"A": 1, "D": 2, "S": 5}, "2": {"A": 1, "D": 0, "S": 5}, "3": {"A": 2, "D": 1, "S": 5}, "4": {"A": 0, "D": 0, "S": 1}, "5": {"A": 2, "D": 2, "S": 5}, "6": {"A": 0, "D": 3, "S": 5}, "7": {"A": 0, "D": 0, "S": 1}, "8": {"A": 0, "D": 0, "S": 1}, "9": {"A": 3, "D": 0, "S": 5}, "10": {"A": 4, "D": 0, "S": 5}, "11": {"A": 2, "D": 0, "S": 5}, "12": {"A": 2, "D": 1, "S": 5}, "13": {"A": 2, "D": 2, "S": 5}, "14": {"A": 0, "D": 0, "S": 1}, "15": {"A": 4, "D": 0, "S": 5}, "16": {"A": 0, "D": 0, "S": 1}, "17": {"A": 3, "D": 0, "S": 5}, "18": {"A": 0, "D": 0, "S": 1}, "19": {"A": 0, "D": 0, "S": 1}, "20": {"A": 1, "D": 2, "S": 5}, "21": {"A": 4, "D": 0, "S": 5}, "22": {"A": 3, "D": 1, "S": 5}, "23": {"A": 2, "D": 2, "S": 5}, "24": {"A": 2, "D": 0, "S": 5}, "25": {"A": 3, "D": 0, "S": 5}, "26": {"A": 0, "D": 0, "S": 0}, "27": {"A": 1, "D": 2, "S": 4}, "28": {"A": 3, "D": 0, "S": 4}, "29": {"A": 1, "D": 1, "S": 4}, "30": {"A": 3, "D": 1, "S": 4}, "31": {"A": 3, "D": 0, "S": 4}, "32": {"A": 3, "D": 0, "S": 4}, "33": {"A": 3, "D": 1, "S": 4}, "34": {"A": 3, "D": 0, "S": 4}, "35": {"A": 4, "D": 0, "S": 4}, "36": {"A": 3, "D": 0, "S": 4}, "37": {"A": 2, "D": 0, "S": 4}, "38": {"A": 1, "D": 2, "S": 4}, "39": {"A": 3, "D": 0, "S": 4}, "40": {"A": 1, "D": 1, "S": 4}, "41": {"A": 0, "D": 2, "S": 4}, "42": {"A": 4, "D": 0, "S": 4}, "43": {"A": 2, "D": 1, "S": 4}, "44": {"A": 2, "D": 0, "S": 4}, "45": {"A": 3, "D": 0, "S": 4}, "46": {"A": 4, "D": 0, "S": 4}, "47": {"A": 1, "D": 0, "S": 4}, "48": {"A": 1, "D": 2, "S": 4}, "49": {"A": 3, "D": 0, "S": 4}, "50": {"A": 3, "D": 0, "S": 4}, "51": {"A": 0, "D": 1, "S": 4}, "52": {"A": 3, "D": 0, "S": 4}, "53": {"A": 3, "D": 0, "S": 4}, "54": {"A": 4, "D": 0, "S": 4}, "55": {"A": 3, "D": 0, "S": 4}, "56": {"A": 0, "D": 0, "S": 4}, "57": {"A": 0, "D": 2, "S": 4}, "58": {"A": 3, "D": 0, "S": 4}, "59": {"A": 3, "D": 0, "S": 4}, "60": {"A": 4, "D": 0, "S": 4}, "61": {"A": 1, "D": 0, "S": 4}, "62": {"A": 3, "D": 0, "S": 4}, "63": {"A": 3, "D": 0, "S": 4}, "64": {"A": 3, "D": 1, "S": 4}, "65": {"A": 4, "D": 0, "S": 4}, "66": {"A": 4, "D": 0, "S": 4}, "67": {"A": 2, "D": 2, "S": 4}, "68": {"A": 4, "D": 0, "S": 4}, "69": {"A": 3, "D": 0, "S": 3}, "70": {"A": 2, "D": 1, "S": 3}, "71": {"A": 2, "D": 0, "S": 3}, "72": {"A": 2, "D": 0, "S": 2}, "73": {"A": 1, "D": 0, "S": 1}}, "n-members": 5}}, "base-clusters": {"x": [-0.22472105650886345, 0.40779709637075595, 0.18902936598940426, -0.015827152334492674, 0.01274190388999949, -0.5392692977958351, 0.47765564707444597, 0.21716868939136574, -0.5493288642912083, 1.0603715856846005, -0.13214244786167223, -1.410118060247752, -1.0789507300531866, 1.3196719764373932, 1.4504812158767078, -1.8613681577978551, 0.5909618499865821, 0.03878242801046458, 0.391015129936025, -1.477610422565824, 1.655255297110118, 0.7797581111396361, -2.261678909751532, -0.006384857803324918, 1.9794073638357446, -0.9965458721966541, -0.7871964349189994, 3.944668083167446, 4.612167929094141, -2.0795698934637183, -0.22942729136024045, -0.36845244416087575, -1.1182493334685848, 1.3003013909711447, 1.2298865096347933, -0.5878823550318389, 0.5084856590179212, 0.2584969853897488, -1.6921728907636597, -0.8457471553275284, -1.1900218669476454, -1.7629000506239043, -1.2768739907435636, -1.2417961893400438, -1.110136953765137, 4.375740108046448, -0.33792128707107877, -1.1791928976990218, -1.9809570469306497, 0.4429903865382543, 0.3777456934152639, 4.048314388141193, -2.373750235569525, -1.5093785648976228, -1.2340288737899876, -0.0347995074294862, 1.313878289334299, 1.1802781001951466, -0.8322191318433724, 0.8591864893799299, 1.4543897037511238, 0.5010384807289284, 0.7751610125319172, -0.9772131910198313, 1.3802892912951048, 0.5187584310886559, 0.987077815998744, 0.22377027647184486, -0.06859163785465218, 0.5780424025541819, 2.1370684300841454, -0.19034375749922988, -1.5734376205229599, 4.361937648557482, 0.062169479170589424, -1.7015981109179796, -3.577161062171128, 1.425963365724268, -1.0449141927504335, -3.1918637571147643, 0.6465086234004146, -1.8664243148702697, -0.32425813916022034], "y": [-0.3777334826044966, 0.5374469370048651, -0.38383520521168385, -1.5281645016968315, 0.5896830707036366, 0.23186534064975203, 0.42569368327003687, 0.5649499119826908, 0.32297948540018767, -2.2603257556040126, 0.4199262417235306, 1.4755561465637375, -0.03688538975231614, 1.8270269572251765, 1.2267139468178265, -1.3952335992469116, 0.4543409164641394, 1.0833226093789186, 0.39329472508762525, -0.41917298411143306, -0.24483767741085055, 0.8753732095134256, -1.1128362542799781, 1.476354411409552, -0.9435697048412794, 1.2913522132994044, 0.0528190156651875, -1.4474132553354115, -3.070987663720197, -0.038139786890744654, 1.3300095170059467, 0.3879871478053895, 0.21114960249883954, 1.2114684645363714, 0.9198198462771255, 0.22047487147953315, 0.5221213553150472, 0.204979116653741, 0.9303532404052466, 0.4367295680484595, 0.7355228774456158, -0.16365408694684988, -0.03909096818363979, -0.07453653555129189, -1.131328205333163, -1.5231451393293782, 1.7262146175182842, 0.3880427059494392, -0.6372141650316713, 0.5601789657233006, -0.112047134607148, -0.32060319621937067, -3.762747141422478, 0.7537340340490432, -0.8632989142650932, 1.297896128141479, 0.9324508894784833, -0.09841263634012797, -0.1518049130292978, 0.20464793581928842, -2.7219030313949735, 1.9443682276857146, -0.14969008972871806, 0.4307528191850082, 0.9580148051958393, -0.2737293205820355, 0.327049875601917, 0.12778033957301335, -0.18852821087873786, -0.19006235549052009, -0.015467002971157943, 0.3721592281985029, -0.30874186576805857, -0.8262792846726988, 0.21629019772472505, -1.0158434761982633, -2.548720312856542, -0.8228243486536408, 0.9071542562451432, -1.6868468426450298, 0.19149166730069495, 0.7057338432204909, 1.7401751484114474], "id": [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82], "count": [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], "members": [[2], [3], [1], [0], [4], [5], [6], [7], [8], [9], [10], [11], [12], [13], [14], [15], [16], [17], [18], [19], [21], [20], [22], [23], [24], [25], [26], [27], [28], [29], [30], [31], [32], [33], [36], [38], [39], [41], [43], [44], [45], [47], [49], [50], [51], [52], [53], [55], [56], [57], [58], [59], [60], [61], [63], [64], [65], [62], [66], [67], [68], [69], [71], [72], [73], [70], [74], [75], [76], [77], [78], [79], [81], [82], [84], [85], [86], [87], [88], [89], [90], [91], [92]]}, "group-clusters": [{"id": 0, "center": [-1.3284956834825883, -0.011552949991391932], "members": [0, 5, 8, 11, 12, 15, 19, 22, 25, 26, 29, 32, 35, 38, 39, 40, 41, 42, 43, 44, 47, 48, 53, 54, 58, 63, 72, 75, 78, 79, 81]}, {"id": 1, "center": [0.5498803525350591, 0.5504828320164177], "members": [1, 2, 4, 6, 7, 10, 13, 14, 16, 17, 18, 20, 21, 23, 30, 31, 33, 34, 36, 37, 46, 49, 50, 55, 56, 57, 59, 61, 62, 64, 65, 66, 67, 68, 69, 70, 71, 74, 77, 80, 82]}, {"id": 2, "center": [3.887039253473742, -1.3553330406863893], "members": [24, 27, 28, 45, 51, 73]}, {"id": 3, "center": [-0.6903954321278842, -2.5643721485949675], "members": [3, 9, 52, 60, 76]}], "subgroup-votes": {"0": {"0": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 0, "D": 7, "S": 11}, "2": {"A": 5, "D": 3, "S": 11}, "3": {"A": 8, "D": 0, "S": 8}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 4, "D": 2, "S": 9}, "6": {"A": 3, "D": 6, "S": 11}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 7, "D": 0, "S": 7}, "10": {"A": 7, "D": 0, "S": 7}, "11": {"A": 5, "D": 0, "S": 8}, "12": {"A": 11, "D": 0, "S": 11}, "13": {"A": 8, "D": 0, "S": 8}, "14": {"A": 0, "D": 0, "S": 0}, "15": {"A": 7, "D": 0, "S": 7}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 7, "D": 0, "S": 7}, "18": {"A": 0, "D": 0, "S": 0}, "19": {"A": 0, "D": 0, "S": 0}, "20": {"A": 3, "D": 2, "S": 9}, "21": {"A": 7, "D": 0, "S": 7}, "22": {"A": 4, "D": 0, "S": 7}, "23": {"A": 5, "D": 1, "S": 8}, "24": {"A": 2, "D": 3, "S": 6}, "25": {"A": 9, "D": 0, "S": 9}, "26": {"A": 0, "D": 0, "S": 0}, "27": {"A": 2, "D": 4, "S": 10}, "28": {"A": 5, "D": 0, "S": 8}, "29": {"A": 6, "D": 0, "S": 9}, "30": {"A": 10, "D": 0, "S": 11}, "31": {"A": 9, "D": 0, "S": 9}, "32": {"A": 4, "D": 0, "S": 7}, "33": {"A": 10, "D": 0, "S": 10}, "34": {"A": 9, "D": 1, "S": 10}, "35": {"A": 9, "D": 0, "S": 9}, "36": {"A": 7, "D": 0, "S": 9}, "37": {"A": 3, "D": 2, "S": 8}, "38": {"A": 5, "D": 2, "S": 8}, "39": {"A": 7, "D": 0, "S": 7}, "40": {"A": 7, "D": 0, "S": 8}, "41": {"A": 1, "D": 2, "S": 6}, "42": {"A": 7, "D": 0, "S": 7}, "43": {"A": 6, "D": 0, "S": 6}, "44": {"A": 6, "D": 0, "S": 6}, "45": {"A": 4, "D": 0, "S": 5}, "46": {"A": 7, "D": 0, "S": 7}, "47": {"A": 3, "D": 0, "S": 5}, "48": {"A": 0, "D": 5, "S": 7}, "49": {"A": 2, "D": 1, "S": 6}, "50": {"A": 3, "D": 0, "S": 6}, "51": {"A": 0, "D": 5, "S": 6}, "52": {"A": 3, "D": 2, "S": 6}, "53": {"A": 8, "D": 0, "S": 8}, "54": {"A": 6, "D": 0, "S": 7}, "55": {"A": 6, "D": 0, "S": 6}, "56": {"A": 3, "D": 3, "S": 7}, "57": {"A": 1, "D": 0, "S": 5}, "58": {"A": 6, "D": 0, "S": 6}, "59": {"A": 4, "D": 0, "S": 5}, "60": {"A": 2, "D": 0, "S": 3}, "61": {"A": 0, "D": 1, "S": 4}, "62": {"A": 1, "D": 0, "S": 3}, "63": {"A": 3, "D": 0, "S": 3}, "64": {"A": 2, "D": 1, "S": 3}, "65": {"A": 1, "D": 1, "S": 2}, "66": {"A": 4, "D": 0, "S": 4}, "67": {"A": 3, "D": 0, "S": 3}, "68": {"A": 2, "D": 0, "S": 3}, "69": {"A": 4, "D": 0, "S": 4}, "70": {"A": 4, "D": 0, "S": 4}, "71": {"A": 1, "D": 0, "S": 2}, "72": {"A": 0, "D": 1, "S": 2}, "73": {"A": 2, "D": 0, "S": 2}}, "n-members": 11}, "1": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 0, "D": 18, "S": 20}, "2": {"A": 9, "D": 5, "S": 20}, "3": {"A": 14, "D": 0, "S": 15}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 12, "D": 0, "S": 15}, "6": {"A": 5, "D": 14, "S": 20}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 15, "D": 0, "S": 15}, "10": {"A": 14, "D": 0, "S": 15}, "11": {"A": 10, "D": 1, "S": 13}, "12": {"A": 15, "D": 0, "S": 16}, "13": {"A": 16, "D": 0, "S": 17}, "14": {"A": 0, "D": 0, "S": 0}, "15": {"A": 15, "D": 0, "S": 15}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 15, "D": 0, "S": 15}, "18": {"A": 0, "D": 0, "S": 0}, "19": {"A": 0, "D": 0, "S": 0}, "20": {"A": 11, "D": 0, "S": 14}, "21": {"A": 15, "D": 0, "S": 15}, "22": {"A": 10, "D": 0, "S": 13}, "23": {"A": 10, "D": 0, "S": 16}, "24": {"A": 5, "D": 5, "S": 14}, "25": {"A": 14, "D": 0, "S": 14}, "26": {"A": 0, "D": 0, "S": 0}, "27": {"A": 9, "D": 1, "S": 14}, "28": {"A": 13, "D": 0, "S": 13}, "29": {"A": 10, "D": 1, "S": 13}, "30": {"A": 13, "D": 0, "S": 17}, "31": {"A": 13, "D": 0, "S": 13}, "32": {"A": 9, "D": 0, "S": 13}, "33": {"A": 15, "D": 0, "S": 15}, "34": {"A": 13, "D": 0, "S": 13}, "35": {"A": 12, "D": 0, "S": 13}, "36": {"A": 8, "D": 0, "S": 11}, "37": {"A": 8, "D": 0, "S": 12}, "38": {"A": 14, "D": 1, "S": 15}, "39": {"A": 12, "D": 0, "S": 14}, "40": {"A": 12, "D": 0, "S": 12}, "41": {"A": 1, "D": 2, "S": 10}, "42": {"A": 14, "D": 0, "S": 14}, "43": {"A": 10, "D": 0, "S": 10}, "44": {"A": 9, "D": 1, "S": 12}, "45": {"A": 7, "D": 1, "S": 10}, "46": {"A": 13, "D": 0, "S": 13}, "47": {"A": 7, "D": 0, "S": 11}, "48": {"A": 4, "D": 4, "S": 11}, "49": {"A": 6, "D": 1, "S": 12}, "50": {"A": 8, "D": 0, "S": 9}, "51": {"A": 7, "D": 0, "S": 11}, "52": {"A": 7, "D": 1, "S": 13}, "53": {"A": 10, "D": 0, "S": 11}, "54": {"A": 11, "D": 0, "S": 11}, "55": {"A": 10, "D": 1, "S": 12}, "56": {"A": 7, "D": 1, "S": 11}, "57": {"A": 1, "D": 0, "S": 6}, "58": {"A": 4, "D": 1, "S": 7}, "59": {"A": 6, "D": 0, "S": 7}, "60": {"A": 4, "D": 0, "S": 6}, "61": {"A": 3, "D": 2, "S": 7}, "62": {"A": 1, "D": 0, "S": 5}, "63": {"A": 7, "D": 0, "S": 7}, "64": {"A": 5, "D": 1, "S": 7}, "65": {"A": 4, "D": 1, "S": 6}, "66": {"A": 6, "D": 0, "S": 7}, "67": {"A": 6, "D": 0, "S": 6}, "68": {"A": 5, "D": 0, "S": 6}, "69": {"A": 6, "D": 0, "S": 7}, "70": {"A": 6, "D": 0, "S": 6}, "71": {"A": 2, "D": 1, "S": 4}, "72": {"A": 3, "D": 2, "S": 6}, "73": {"A": 1, "D": 0, "S": 3}}, "n-members": 20}}, "1": {"0": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 0, "D": 6, "S": 7}, "2": {"A": 1, "D": 3, "S": 7}, "3": {"A": 7, "D": 0, "S": 7}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 5, "D": 0, "S": 5}, "6": {"A": 2, "D": 4, "S": 7}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 5, "D": 0, "S": 5}, "10": {"A": 5, "D": 0, "S": 5}, "11": {"A": 1, "D": 1, "S": 5}, "12": {"A": 7, "D": 0, "S": 7}, "13": {"A": 5, "D": 0, "S": 5}, "14": {"A": 0, "D": 0, "S": 0}, "15": {"A": 5, "D": 0, "S": 5}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 6, "D": 0, "S": 6}, "18": {"A": 0, "D": 0, "S": 0}, "19": {"A": 0, "D": 0, "S": 0}, "20": {"A": 6, "D": 0, "S": 6}, "21": {"A": 4, "D": 1, "S": 5}, "22": {"A": 0, "D": 3, "S": 5}, "23": {"A": 6, "D": 0, "S": 6}, "24": {"A": 1, "D": 3, "S": 5}, "25": {"A": 6, "D": 0, "S": 6}, "26": {"A": 0, "D": 0, "S": 0}, "27": {"A": 6, "D": 0, "S": 6}, "28": {"A": 6, "D": 0, "S": 6}, "29": {"A": 4, "D": 0, "S": 5}, "30": {"A": 1, "D": 5, "S": 7}, "31": {"A": 6, "D": 0, "S": 6}, "32": {"A": 5, "D": 0, "S": 5}, "33": {"A": 1, "D": 2, "S": 6}, "34": {"A": 5, "D": 0, "S": 5}, "35": {"A": 4, "D": 0, "S": 4}, "36": {"A": 4, "D": 0, "S": 4}, "37": {"A": 6, "D": 0, "S": 6}, "38": {"A": 4, "D": 1, "S": 5}, "39": {"A": 2, "D": 1, "S": 4}, "40": {"A": 3, "D": 0, "S": 3}, "41": {"A": 3, "D": 0, "S": 3}, "42": {"A": 3, "D": 0, "S": 3}, "43": {"A": 2, "D": 0, "S": 3}, "44": {"A": 3, "D": 0, "S": 3}, "45": {"A": 2, "D": 1, "S": 3}, "46": {"A": 3, "D": 0, "S": 4}, "47": {"A": 3, "D": 0, "S": 3}, "48": {"A": 2, "D": 0, "S": 3}, "49": {"A": 0, "D": 2, "S": 3}, "50": {"A": 1, "D": 0, "S": 3}, "51": {"A": 3, "D": 0, "S": 4}, "52": {"A": 1, "D": 0, "S": 2}, "53": {"A": 3, "D": 0, "S": 3}, "54": {"A": 3, "D": 0, "S": 3}, "55": {"A": 3, "D": 0, "S": 4}, "56": {"A": 2, "D": 0, "S": 2}, "57": {"A": 1, "D": 0, "S": 1}, "58": {"A": 0, "D": 1, "S": 2}, "59": {"A": 0, "D": 0, "S": 1}, "60": {"A": 2, "D": 0, "S": 2}, "61": {"A": 0, "D": 0, "S": 1}, "62": {"A": 1, "D": 0, "S": 1}, "63": {"A": 1, "D": 0, "S": 1}, "64": {"A": 3, "D": 0, "S": 3}, "65": {"A": 1, "D": 0, "S": 1}, "66": {"A": 2, "D": 0, "S": 2}, "67": {"A": 1, "D": 0, "S": 2}, "68": {"A": 1, "D": 0, "S": 3}, "69": {"A": 2, "D": 1, "S": 3}, "70": {"A": 2, "D": 0, "S": 3}, "71": {"A": 0, "D": 1, "S": 2}, "72": {"A": 1, "D": 0, "S": 2}, "73": {"A": 1, "D": 0, "S": 1}}, "n-members": 7}, "1": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 0, "D": 6, "S": 8}, "2": {"A": 2, "D": 4, "S": 8}, "3": {"A": 3, "D": 0, "S": 3}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 2, "D": 0, "S": 3}, "6": {"A": 0, "D": 6, "S": 8}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 2, "D": 0, "S": 2}, "10": {"A": 2, "D": 0, "S": 2}, "11": {"A": 1, "D": 0, "S": 2}, "12": {"A": 3, "D": 0, "S": 4}, "13": {"A": 2, "D": 0, "S": 3}, "14": {"A": 0, "D": 0, "S": 0}, "15": {"A": 3, "D": 0, "S": 3}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 4, "D": 0, "S": 4}, "18": {"A": 0, "D": 0, "S": 0}, "19": {"A": 0, "D": 0, "S": 0}, "20": {"A": 2, "D": 0, "S": 2}, "21": {"A": 4, "D": 0, "S": 4}, "22": {"A": 0, "D": 1, "S": 2}, "23": {"A": 1, "D": 0, "S": 2}, "24": {"A": 0, "D": 1, "S": 2}, "25": {"A": 4, "D": 0, "S": 4}, "26": {"A": 1, "D": 0, "S": 1}, "27": {"A": 2, "D": 0, "S": 2}, "28": {"A": 1, "D": 0, "S": 2}, "29": {"A": 2, "D": 0, "S": 3}, "30": {"A": 1, "D": 5, "S": 8}, "31": {"A": 2, "D": 0, "S": 2}, "32": {"A": 1, "D": 0, "S": 2}, "33": {"A": 2, "D": 0, "S": 3}, "34": {"A": 4, "D": 0, "S": 5}, "35": {"A": 2, "D": 0, "S": 2}, "36": {"A": 2, "D": 1, "S": 3}, "37": {"A": 2, "D": 0, "S": 2}, "38": {"A": 2, "D": 0, "S": 3}, "39": {"A": 2, "D": 1, "S": 3}, "40": {"A": 3, "D": 0, "S": 3}, "41": {"A": 1, "D": 0, "S": 2}, "42": {"A": 2, "D": 0, "S": 3}, "43": {"A": 1, "D": 1, "S": 2}, "44": {"A": 1, "D": 0, "S": 2}, "45": {"A": 2, "D": 0, "S": 2}, "46": {"A": 1, "D": 0, "S": 2}, "47": {"A": 0, "D": 1, "S": 2}, "48": {"A": 1, "D": 0, "S": 4}, "49": {"A": 0, "D": 1, "S": 2}, "50": {"A": 0, "D": 0, "S": 2}, "51": {"A": 0, "D": 2, "S": 3}, "52": {"A": 1, "D": 0, "S": 3}, "53": {"A": 2, "D": 2, "S": 4}, "54": {"A": 3, "D": 0, "S": 3}, "55": {"A": 1, "D": 1, "S": 3}, "56": {"A": 2, "D": 2, "S": 5}, "57": {"A": 0, "D": 0, "S": 2}, "58": {"A": 1, "D": 1, "S": 3}, "59": {"A": 3, "D": 0, "S": 3}, "60": {"A": 2, "D": 0, "S": 3}, "61": {"A": 1, "D": 2, "S": 3}, "62": {"A": 1, "D": 0, "S": 2}, "63": {"A": 5, "D": 0, "S": 6}, "64": {"A": 5, "D": 0, "S": 5}, "65": {"A": 2, "D": 1, "S": 3}, "66": {"A": 2, "D": 0, "S": 3}, "67": {"A": 1, "D": 1, "S": 2}, "68": {"A": 3, "D": 1, "S": 4}, "69": {"A": 1, "D": 0, "S": 2}, "70": {"A": 2, "D": 0, "S": 3}, "71": {"A": 3, "D": 1, "S": 5}, "72": {"A": 4, "D": 0, "S": 6}, "73": {"A": 1, "D": 0, "S": 3}}, "n-members": 9}, "2": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 1, "D": 9, "S": 14}, "2": {"A": 2, "D": 2, "S": 13}, "3": {"A": 11, "D": 0, "S": 11}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 7, "D": 0, "S": 9}, "6": {"A": 2, "D": 11, "S": 14}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 9, "D": 1, "S": 10}, "10": {"A": 11, "D": 0, "S": 12}, "11": {"A": 4, "D": 1, "S": 9}, "12": {"A": 10, "D": 0, "S": 11}, "13": {"A": 9, "D": 0, "S": 9}, "14": {"A": 0, "D": 0, "S": 0}, "15": {"A": 9, "D": 0, "S": 9}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 10, "D": 0, "S": 10}, "18": {"A": 0, "D": 0, "S": 0}, "19": {"A": 0, "D": 0, "S": 0}, "20": {"A": 9, "D": 0, "S": 11}, "21": {"A": 11, "D": 0, "S": 11}, "22": {"A": 0, "D": 3, "S": 9}, "23": {"A": 6, "D": 0, "S": 8}, "24": {"A": 4, "D": 2, "S": 9}, "25": {"A": 8, "D": 0, "S": 9}, "26": {"A": 0, "D": 0, "S": 0}, "27": {"A": 6, "D": 0, "S": 6}, "28": {"A": 5, "D": 0, "S": 6}, "29": {"A": 3, "D": 1, "S": 5}, "30": {"A": 1, "D": 4, "S": 7}, "31": {"A": 3, "D": 0, "S": 4}, "32": {"A": 1, "D": 0, "S": 5}, "33": {"A": 3, "D": 1, "S": 5}, "34": {"A": 7, "D": 0, "S": 7}, "35": {"A": 4, "D": 0, "S": 4}, "36": {"A": 4, "D": 0, "S": 5}, "37": {"A": 5, "D": 0, "S": 6}, "38": {"A": 5, "D": 0, "S": 6}, "39": {"A": 5, "D": 0, "S": 6}, "40": {"A": 5, "D": 0, "S": 5}, "41": {"A": 2, "D": 1, "S": 4}, "42": {"A": 6, "D": 0, "S": 6}, "43": {"A": 4, "D": 0, "S": 5}, "44": {"A": 6, "D": 0, "S": 7}, "45": {"A": 5, "D": 0, "S": 5}, "46": {"A": 5, "D": 0, "S": 5}, "47": {"A": 4, "D": 0, "S": 4}, "48": {"A": 3, "D": 1, "S": 4}, "49": {"A": 1, "D": 1, "S": 5}, "50": {"A": 2, "D": 0, "S": 5}, "51": {"A": 1, "D": 0, "S": 4}, "52": {"A": 2, "D": 1, "S": 3}, "53": {"A": 3, "D": 0, "S": 5}, "54": {"A": 4, "D": 0, "S": 4}, "55": {"A": 5, "D": 0, "S": 5}, "56": {"A": 2, "D": 0, "S": 4}, "57": {"A": 0, "D": 0, "S": 3}, "58": {"A": 1, "D": 1, "S": 3}, "59": {"A": 4, "D": 0, "S": 4}, "60": {"A": 3, "D": 1, "S": 4}, "61": {"A": 2, "D": 0, "S": 3}, "62": {"A": 0, "D": 0, "S": 3}, "63": {"A": 4, "D": 0, "S": 4}, "64": {"A": 0, "D": 0, "S": 2}, "65": {"A": 1, "D": 0, "S": 2}, "66": {"A": 2, "D": 0, "S": 2}, "67": {"A": 0, "D": 0, "S": 2}, "68": {"A": 3, "D": 0, "S": 3}, "69": {"A": 1, "D": 0, "S": 1}, "70": {"A": 1, "D": 0, "S": 1}, "71": {"A": 0, "D": 0, "S": 1}, "72": {"A": 0, "D": 0, "S": 1}, "73": {"A": 0, "D": 0, "S": 1}}, "n-members": 14}, "3": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 0, "D": 4, "S": 4}, "2": {"A": 1, "D": 2, "S": 4}, "3": {"A": 2, "D": 0, "S": 2}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 3, "D": 0, "S": 3}, "6": {"A": 2, "D": 2, "S": 4}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 2, "D": 0, "S": 2}, "10": {"A": 2, "D": 0, "S": 2}, "11": {"A": 0, "D": 0, "S": 2}, "12": {"A": 1, "D": 0, "S": 2}, "13": {"A": 2, "D": 0, "S": 3}, "14": {"A": 0, "D": 0, "S": 0}, "15": {"A": 2, "D": 0, "S": 2}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 2, "D": 0, "S": 2}, "18": {"A": 0, "D": 0, "S": 0}, "19": {"A": 0, "D": 0, "S": 0}, "20": {"A": 1, "D": 0, "S": 2}, "21": {"A": 2, "D": 0, "S": 2}, "22": {"A": 0, "D": 1, "S": 3}, "23": {"A": 1, "D": 0, "S": 2}, "24": {"A": 0, "D": 0, "S": 2}, "25": {"A": 1, "D": 0, "S": 2}, "26": {"A": 0, "D": 0, "S": 0}, "27": {"A": 2, "D": 0, "S": 2}, "28": {"A": 1, "D": 1, "S": 3}, "29": {"A": 1, "D": 0, "S": 3}, "30": {"A": 1, "D": 3, "S": 4}, "31": {"A": 2, "D": 1, "S": 3}, "32": {"A": 1, "D": 0, "S": 2}, "33": {"A": 3, "D": 0, "S": 4}, "34": {"A": 1, "D": 0, "S": 3}, "35": {"A": 1, "D": 0, "S": 3}, "36": {"A": 2, "D": 0, "S": 2}, "37": {"A": 2, "D": 0, "S": 3}, "38": {"A": 0, "D": 1, "S": 2}, "39": {"A": 1, "D": 0, "S": 2}, "40": {"A": 2, "D": 0, "S": 2}, "41": {"A": 2, "D": 0, "S": 2}, "42": {"A": 2, "D": 0, "S": 2}, "43": {"A": 2, "D": 0, "S": 2}, "44": {"A": 2, "D": 0, "S": 2}, "45": {"A": 2, "D": 0, "S": 3}, "46": {"A": 2, "D": 0, "S": 2}, "47": {"A": 3, "D": 0, "S": 3}, "48": {"A": 0, "D": 0, "S": 2}, "49": {"A": 1, "D": 0, "S": 2}, "50": {"A": 1, "D": 0, "S": 2}, "51": {"A": 0, "D": 0, "S": 2}, "52": {"A": 0, "D": 1, "S": 2}, "53": {"A": 3, "D": 0, "S": 3}, "54": {"A": 2, "D": 0, "S": 2}, "55": {"A": 1, "D": 0, "S": 2}, "56": {"A": 2, "D": 0, "S": 2}, "57": {"A": 1, "D": 0, "S": 2}, "58": {"A": 0, "D": 1, "S": 2}, "59": {"A": 1, "D": 0, "S": 3}, "60": {"A": 1, "D": 0, "S": 2}, "61": {"A": 2, "D": 0, "S": 3}, "62": {"A": 0, "D": 0, "S": 2}, "63": {"A": 2, "D": 0, "S": 3}, "64": {"A": 2, "D": 0, "S": 2}, "65": {"A": 2, "D": 0, "S": 2}, "66": {"A": 3, "D": 0, "S": 3}, "67": {"A": 1, "D": 0, "S": 2}, "68": {"A": 1, "D": 0, "S": 3}, "69": {"A": 0, "D": 3, "S": 3}, "70": {"A": 0, "D": 3, "S": 3}, "71": {"A": 0, "D": 0, "S": 2}, "72": {"A": 1, "D": 0, "S": 2}, "73": {"A": 1, "D": 0, "S": 2}}, "n-members": 4}, "4": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 0, "D": 5, "S": 7}, "2": {"A": 2, "D": 4, "S": 7}, "3": {"A": 7, "D": 0, "S": 7}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 7, "D": 0, "S": 7}, "6": {"A": 2, "D": 3, "S": 7}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 7, "D": 0, "S": 7}, "10": {"A": 7, "D": 0, "S": 7}, "11": {"A": 5, "D": 1, "S": 7}, "12": {"A": 7, "D": 0, "S": 7}, "13": {"A": 7, "D": 0, "S": 7}, "14": {"A": 0, "D": 0, "S": 0}, "15": {"A": 7, "D": 0, "S": 7}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 7, "D": 0, "S": 7}, "18": {"A": 0, "D": 0, "S": 0}, "19": {"A": 0, "D": 0, "S": 0}, "20": {"A": 7, "D": 0, "S": 7}, "21": {"A": 7, "D": 0, "S": 7}, "22": {"A": 3, "D": 2, "S": 7}, "23": {"A": 7, "D": 0, "S": 7}, "24": {"A": 0, "D": 3, "S": 7}, "25": {"A": 7, "D": 0, "S": 7}, "26": {"A": 0, "D": 0, "S": 0}, "27": {"A": 7, "D": 0, "S": 7}, "28": {"A": 7, "D": 0, "S": 7}, "29": {"A": 6, "D": 0, "S": 7}, "30": {"A": 2, "D": 2, "S": 7}, "31": {"A": 7, "D": 0, "S": 7}, "32": {"A": 7, "D": 0, "S": 7}, "33": {"A": 7, "D": 0, "S": 7}, "34": {"A": 7, "D": 0, "S": 7}, "35": {"A": 7, "D": 0, "S": 7}, "36": {"A": 7, "D": 0, "S": 7}, "37": {"A": 7, "D": 0, "S": 7}, "38": {"A": 6, "D": 0, "S": 6}, "39": {"A": 4, "D": 1, "S": 5}, "40": {"A": 5, "D": 0, "S": 5}, "41": {"A": 1, "D": 1, "S": 5}, "42": {"A": 4, "D": 0, "S": 4}, "43": {"A": 3, "D": 0, "S": 4}, "44": {"A": 4, "D": 0, "S": 4}, "45": {"A": 4, "D": 0, "S": 4}, "46": {"A": 4, "D": 0, "S": 4}, "47": {"A": 4, "D": 0, "S": 4}, "48": {"A": 1, "D": 1, "S": 4}, "49": {"A": 3, "D": 1, "S": 4}, "50": {"A": 2, "D": 1, "S": 4}, "51": {"A": 4, "D": 0, "S": 4}, "52": {"A": 1, "D": 2, "S": 4}, "53": {"A": 4, "D": 0, "S": 4}, "54": {"A": 4, "D": 0, "S": 4}, "55": {"A": 4, "D": 0, "S": 4}, "56": {"A": 3, "D": 1, "S": 4}, "57": {"A": 3, "D": 0, "S": 4}, "58": {"A": 1, "D": 2, "S": 4}, "59": {"A": 2, "D": 2, "S": 4}, "60": {"A": 4, "D": 0, "S": 4}, "61": {"A": 3, "D": 0, "S": 3}, "62": {"A": 1, "D": 1, "S": 3}, "63": {"A": 3, "D": 0, "S": 3}, "64": {"A": 3, "D": 0, "S": 3}, "65": {"A": 2, "D": 0, "S": 3}, "66": {"A": 2, "D": 0, "S": 3}, "67": {"A": 2, "D": 0, "S": 3}, "68": {"A": 3, "D": 0, "S": 3}, "69": {"A": 2, "D": 0, "S": 3}, "70": {"A": 2, "D": 0, "S": 3}, "71": {"A": 0, "D": 0, "S": 1}, "72": {"A": 1, "D": 0, "S": 1}, "73": {"A": 1, "D": 0, "S": 1}}, "n-members": 7}}, "2": {"0": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 0, "D": 1, "S": 1}, "2": {"A": 0, "D": 1, "S": 1}, "3": {"A": 1, "D": 0, "S": 1}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 1, "D": 0, "S": 1}, "6": {"A": 0, "D": 1, "S": 1}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 0, "D": 0, "S": 1}, "10": {"A": 0, "D": 0, "S": 1}, "11": {"A": 0, "D": 0, "S": 1}, "12": {"A": 0, "D": 0, "S": 1}, "13": {"A": 1, "D": 0, "S": 1}, "14": {"A": 0, "D": 0, "S": 0}, "15": {"A": 1, "D": 0, "S": 1}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 1, "D": 0, "S": 1}, "18": {"A": 0, "D": 0, "S": 0}, "19": {"A": 0, "D": 0, "S": 0}, "20": {"A": 1, "D": 0, "S": 1}, "21": {"A": 1, "D": 0, "S": 1}, "22": {"A": 0, "D": 0, "S": 1}, "23": {"A": 1, "D": 0, "S": 1}, "24": {"A": 0, "D": 0, "S": 1}, "25": {"A": 1, "D": 0, "S": 1}, "26": {"A": 0, "D": 0, "S": 0}, "27": {"A": 1, "D": 0, "S": 1}, "28": {"A": 0, "D": 0, "S": 1}, "29": {"A": 1, "D": 0, "S": 1}, "30": {"A": 0, "D": 1, "S": 1}, "31": {"A": 1, "D": 0, "S": 1}, "32": {"A": 1, "D": 0, "S": 1}, "33": {"A": 0, "D": 0, "S": 1}, "34": {"A": 0, "D": 0, "S": 1}, "35": {"A": 1, "D": 0, "S": 1}, "36": {"A": 1, "D": 0, "S": 1}, "37": {"A": 0, "D": 0, "S": 1}, "38": {"A": 0, "D": 1, "S": 1}, "39": {"A": 0, "D": 0, "S": 0}, "40": {"A": 0, "D": 0, "S": 0}, "41": {"A": 0, "D": 0, "S": 0}, "42": {"A": 0, "D": 0, "S": 0}, "43": {"A": 0, "D": 0, "S": 0}, "44": {"A": 0, "D": 0, "S": 0}, "45": {"A": 0, "D": 0, "S": 0}, "46": {"A": 0, "D": 0, "S": 0}, "47": {"A": 0, "D": 0, "S": 0}, "48": {"A": 0, "D": 0, "S": 0}, "49": {"A": 0, "D": 0, "S": 0}, "50": {"A": 0, "D": 0, "S": 0}, "51": {"A": 0, "D": 0, "S": 0}, "52": {"A": 0, "D": 0, "S": 0}, "53": {"A": 0, "D": 0, "S": 0}, "54": {"A": 0, "D": 0, "S": 0}, "55": {"A": 0, "D": 0, "S": 0}, "56": {"A": 0, "D": 0, "S": 0}, "57": {"A": 0, "D": 0, "S": 0}, "58": {"A": 0, "D": 0, "S": 0}, "59": {"A": 0, "D": 0, "S": 0}, "60": {"A": 0, "D": 0, "S": 0}, "61": {"A": 0, "D": 0, "S": 0}, "62": {"A": 0, "D": 0, "S": 0}, "63": {"A": 0, "D": 0, "S": 0}, "64": {"A": 0, "D": 0, "S": 0}, "65": {"A": 0, "D": 0, "S": 0}, "66": {"A": 0, "D": 0, "S": 0}, "67": {"A": 0, "D": 0, "S": 0}, "68": {"A": 0, "D": 0, "S": 0}, "69": {"A": 0, "D": 0, "S": 0}, "70": {"A": 0, "D": 0, "S": 0}, "71": {"A": 0, "D": 0, "S": 0}, "72": {"A": 0, "D": 0, "S": 0}, "73": {"A": 0, "D": 0, "S": 0}}, "n-members": 1}, "1": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 0, "D": 2, "S": 5}, "2": {"A": 1, "D": 2, "S": 5}, "3": {"A": 5, "D": 0, "S": 5}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 5, "D": 0, "S": 5}, "6": {"A": 1, "D": 2, "S": 5}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 3, "D": 0, "S": 5}, "10": {"A": 5, "D": 0, "S": 5}, "11": {"A": 1, "D": 1, "S": 5}, "12": {"A": 2, "D": 1, "S": 5}, "13": {"A": 0, "D": 2, "S": 5}, "14": {"A": 0, "D": 0, "S": 0}, "15": {"A": 4, "D": 0, "S": 5}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 5, "D": 0, "S": 5}, "18": {"A": 0, "D": 0, "S": 0}, "19": {"A": 0, "D": 0, "S": 0}, "20": {"A": 5, "D": 0, "S": 5}, "21": {"A": 4, "D": 0, "S": 5}, "22": {"A": 0, "D": 4, "S": 5}, "23": {"A": 5, "D": 0, "S": 5}, "24": {"A": 0, "D": 3, "S": 5}, "25": {"A": 3, "D": 0, "S": 5}, "26": {"A": 0, "D": 0, "S": 0}, "27": {"A": 5, "D": 0, "S": 5}, "28": {"A": 3, "D": 1, "S": 5}, "29": {"A": 4, "D": 1, "S": 5}, "30": {"A": 0, "D": 5, "S": 5}, "31": {"A": 2, "D": 1, "S": 5}, "32": {"A": 4, "D": 1, "S": 5}, "33": {"A": 0, "D": 5, "S": 5}, "34": {"A": 0, "D": 4, "S": 5}, "35": {"A": 1, "D": 3, "S": 5}, "36": {"A": 4, "D": 0, "S": 5}, "37": {"A": 5, "D": 0, "S": 5}, "38": {"A": 0, "D": 3, "S": 5}, "39": {"A": 2, "D": 2, "S": 5}, "40": {"A": 1, "D": 0, "S": 3}, "41": {"A": 3, "D": 0, "S": 3}, "42": {"A": 3, "D": 0, "S": 3}, "43": {"A": 0, "D": 1, "S": 3}, "44": {"A": 1, "D": 0, "S": 3}, "45": {"A": 3, "D": 0, "S": 3}, "46": {"A": 1, "D": 1, "S": 3}, "47": {"A": 2, "D": 0, "S": 3}, "48": {"A": 3, "D": 0, "S": 3}, "49": {"A": 0, "D": 0, "S": 3}, "50": {"A": 1, "D": 0, "S": 3}, "51": {"A": 1, "D": 1, "S": 3}, "52": {"A": 1, "D": 0, "S": 3}, "53": {"A": 2, "D": 1, "S": 3}, "54": {"A": 2, "D": 0, "S": 3}, "55": {"A": 3, "D": 0, "S": 3}, "56": {"A": 1, "D": 0, "S": 3}, "57": {"A": 0, "D": 0, "S": 3}, "58": {"A": 0, "D": 2, "S": 3}, "59": {"A": 0, "D": 1, "S": 3}, "60": {"A": 3, "D": 0, "S": 3}, "61": {"A": 3, "D": 0, "S": 3}, "62": {"A": 0, "D": 0, "S": 2}, "63": {"A": 2, "D": 0, "S": 2}, "64": {"A": 2, "D": 0, "S": 2}, "65": {"A": 1, "D": 0, "S": 2}, "66": {"A": 1, "D": 0, "S": 2}, "67": {"A": 2, "D": 0, "S": 2}, "68": {"A": 2, "D": 0, "S": 2}, "69": {"A": 0, "D": 2, "S": 2}, "70": {"A": 0, "D": 2, "S": 2}, "71": {"A": 0, "D": 1, "S": 1}, "72": {"A": 2, "D": 0, "S": 2}, "73": {"A": 2, "D": 0, "S": 2}}, "n-members": 5}}, "3": {"0": {"votes": {"0": {"A": 0, "D": 0, "S": 0}, "1": {"A": 1, "D": 1, "S": 2}, "2": {"A": 1, "D": 0, "S": 2}, "3": {"A": 1, "D": 0, "S": 2}, "4": {"A": 0, "D": 0, "S": 0}, "5": {"A": 0, "D": 2, "S": 2}, "6": {"A": 0, "D": 2, "S": 2}, "7": {"A": 0, "D": 0, "S": 0}, "8": {"A": 0, "D": 0, "S": 0}, "9": {"A": 2, "D": 0, "S": 2}, "10": {"A": 2, "D": 0, "S": 2}, "11": {"A": 1, "D": 0, "S": 2}, "12": {"A": 1, "D": 0, "S": 2}, "13": {"A": 2, "D": 0, "S": 2}, "14": {"A": 0, "D": 0, "S": 0}, "15": {"A": 2, "D": 0, "S": 2}, "16": {"A": 0, "D": 0, "S": 0}, "17": {"A": 2, "D": 0, "S": 2}, "18": {"A": 0, "D": 0, "S": 0}, "19": {"A": 0, "D": 0, "S": 0}, "20": {"A": 0, "D": 1, "S": 2}, "21": {"A": 2, "D": 0, "S": 2}, "22": {"A": 2, "D": 0, "S": 2}, "23": {"A": 0, "D": 2, "S": 2}, "24": {"A": 0, "D": 0, "S": 2}, "25": {"A": 1, "D": 0, "S": 2}, "26": {"A": 0, "D": 0, "S": 0}, "27": {"A": 0, "D": 2, "S": 2}, "28": {"A": 2, "D": 0, "S": 2}, "29": {"A": 0, "D": 1, "S": 2}, "30": {"A": 2, "D": 0, "S": 2}, "31": {"A": 2, "D": 0, "S": 2}, "32": {"A": 1, "D": 0, "S": 2}, "33": {"A": 2, "D": 0, "S": 2}, "34": {"A": 2, "D": 0, "S": 2}, "35": {"A": 2, "D": 0, "S": 2}, "36": {"A": 2, "D": 0, "S": 2}, "37": {"A": 1, "D": 0, "S": 2}, "38": {"A": 1, "D": 1, "S": 2}, "39": {"A": 2, "D": 0, "S": 2}, "40": {"A": 1, "D": 0, "S": 2}, "41": {"A": 0, "D": 2, "S": 2}, "42": {"A": 2, "D": 0, "S": 2}, "43": {"A": 1, "D": 0, "S": 2}, "44": {"A": 1, "D": 0, "S": 2}, "45": {"A": 2, "D": 0, "S": 2}, "46": {"A": 2, "D": 0, "S": 2}, "47": {"A": 1, "D": 0, "S": 2}, "48": {"A": 0, "D": 2, "S": 2}, "49": {"A": 2, "D": 0, "S": 2}, "50": {"A": 2, "D": 0, "S": 2}, "51": {"A": 0, "D": 1, "S": 2}, "52": {"A": 2, "D": 0, "S": 2}, "53": {"A": 2, "D": 0, "S": 2}, "54": {"A": 2, "D": 0, "S": 2}, "55": {"A": 2, "D": 0, "S": 2}, "56": {"A": 0, "D": 0, "S": 2}, "57": {"A": 0, "D": 2, "S": 2}, "58": {"A": 2, "D": 0, "S": 2}, "59": {"A": 2, "D": 0, "S": 2}, "60": {"A": 2, "D": 0, "S": 2}, "61": {"A": 1, "D": 0, "S": 2}, "62": {"A": 2, "D": 0, "S": 2}, "63": {"A": 2, "D": 0, "S": 2}, "64": {"A": 1, "D": 1, "S": 2}, "65": {"A": 2, "D": 0, "S": 2}, "66": {"A": 2, "D": 0, "S": 2}, "67": {"A": 1, "D": 1, "S": 2}, "68": {"A": 2, "D": 0, "S": 2}, "69": {"A": 1, "D": 0, "S": 1}, "70": {"A": 1, "D": 0, "S": 1}, "71": {"A": 0, "D": 0, "S": 1}, "72": {"A": 1, "D": 0, "S": 1}, "73": {"A": 1, "D": 0, "S": 1}}, "n-members": 2}, "1": {"votes": {"0": {"A": 0, "D": 0, "S": 1}, "1": {"A": 0, "D": 1, "S": 3}, "2": {"A": 0, "D": 0, "S": 3}, "3": {"A": 1, "D": 1, "S": 3}, "4": {"A": 0, "D": 0, "S": 1}, "5": {"A": 2, "D": 0, "S": 3}, "6": {"A": 0, "D": 1, "S": 3}, "7": {"A": 0, "D": 0, "S": 1}, "8": {"A": 0, "D": 0, "S": 1}, "9": {"A": 1, "D": 0, "S": 3}, "10": {"A": 2, "D": 0, "S": 3}, "11": {"A": 1, "D": 0, "S": 3}, "12": {"A": 1, "D": 1, "S": 3}, "13": {"A": 0, "D": 2, "S": 3}, "14": {"A": 0, "D": 0, "S": 1}, "15": {"A": 2, "D": 0, "S": 3}, "16": {"A": 0, "D": 0, "S": 1}, "17": {"A": 1, "D": 0, "S": 3}, "18": {"A": 0, "D": 0, "S": 1}, "19": {"A": 0, "D": 0, "S": 1}, "20": {"A": 1, "D": 1, "S": 3}, "21": {"A": 2, "D": 0, "S": 3}, "22": {"A": 1, "D": 1, "S": 3}, "23": {"A": 2, "D": 0, "S": 3}, "24": {"A": 2, "D": 0, "S": 3}, "25": {"A": 2, "D": 0, "S": 3}, "26": {"A": 0, "D": 0, "S": 0}, "27": {"A": 1, "D": 0, "S": 2}, "28": {"A": 1, "D": 0, "S": 2}, "29": {"A": 1, "D": 0, "S": 2}, "30": {"A": 1, "D": 1, "S": 2}, "31": {"A": 1, "D": 0, "S": 2}, "32": {"A": 2, "D": 0, "S": 2}, "33": {"A": 1, "D": 1, "S": 2}, "34": {"A": 1, "D": 0, "S": 2}, "35": {"A": 2, "D": 0, "S": 2}, "36": {"A": 1, "D": 0, "S": 2}, "37": {"A": 1, "D": 0, "S": 2}, "38": {"A": 0, "D": 1, "S": 2}, "39": {"A": 1, "D": 0, "S": 2}, "40": {"A": 0, "D": 1, "S": 2}, "41": {"A": 0, "D": 0, "S": 2}, "42": {"A": 2, "D": 0, "S": 2}, "43": {"A": 1, "D": 1, "S": 2}, "44": {"A": 1, "D": 0, "S": 2}, "45": {"A": 1, "D": 0, "S": 2}, "46": {"A": 2, "D": 0, "S": 2}, "47": {"A": 0, "D": 0, "S": 2}, "48": {"A": 1, "D": 0, "S": 2}, "49": {"A": 1, "D": 0, "S": 2}, "50": {"A": 1, "D": 0, "S": 2}, "51": {"A": 0, "D": 0, "S": 2}, "52": {"A": 1, "D": 0, "S": 2}, "53": {"A": 1, "D": 0, "S": 2}, "54": {"A": 2, "D": 0, "S": 2}, "55": {"A": 1, "D": 0, "S": 2}, "56": {"A": 0, "D": 0, "S": 2}, "57": {"A": 0, "D": 0, "S": 2}, "58": {"A": 1, "D": 0, "S": 2}, "59": {"A": 1, "D": 0, "S": 2}, "60": {"A": 2, "D": 0, "S": 2}, "61": {"A": 0, "D": 0, "S": 2}, "62": {"A": 1, "D": 0, "S": 2}, "63": {"A": 1, "D": 0, "S": 2}, "64": {"A": 2, "D": 0, "S": 2}, "65": {"A": 2, "D": 0, "S": 2}, "66": {"A": 2, "D": 0, "S": 2}, "67": {"A": 1, "D": 1, "S": 2}, "68": {"A": 2, "D": 0, "S": 2}, "69": {"A": 2, "D": 0, "S": 2}, "70": {"A": 1, "D": 1, "S": 2}, "71": {"A": 2, "D": 0, "S": 2}, "72": {"A": 1, "D": 0, "S": 1}, "73": {"A": 0, "D": 0, "S": 0}}, "n-members": 3}}}, "lastModTimestamp": null, "subgroup-repness": {"0": {"0": [{"tid": 58, "p-test": 2.6457513110645907, "n-agree": 6, "repness": 1.575, "n-trials": 6, "n-success": 6, "p-success": 0.875, "best-agree": true, "repful-for": "agree", "repness-test": 1.8114221}, {"tid": 44, "p-test": 2.6457513110645907, "repness": 1.225, "n-trials": 6, "n-success": 6, "p-success": 0.875, "repful-for": "agree", "repness-test": 1.378569}, {"tid": 73, "p-test": 1.7320508075688772, "repness": 1.875, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "agree", "repness-test": 1.4491377}, {"tid": 51, "p-test": 1.8898223650461359, "repness": 9.75, "n-trials": 6, "n-success": 5, "p-success": 0.75, "repful-for": "disagree", "repness-test": 3.3729575}, {"tid": 48, "p-test": 1.4142135623730951, "repness": 1.733333333333333, "n-trials": 7, "n-success": 5, "p-success": 0.6666666666666667, "repful-for": "disagree", "repness-test": 1.4679517}], "1": [{"tid": 28, "p-test": 3.7416573867739413, "n-agree": 13, "repness": 1.555555555555556, "n-trials": 13, "n-success": 13, "p-success": 0.9333333333333333, "best-agree": true, "repful-for": "agree", "repness-test": 2.3166068}, {"tid": 38, "p-test": 3.5, "repness": 1.470588235294118, "n-trials": 15, "n-success": 14, "p-success": 0.8823529411764706, "repful-for": "agree", "repness-test": 1.7730204}, {"tid": 20, "p-test": 2.3237900077244507, "repness": 2.0625, "n-trials": 14, "n-success": 11, "p-success": 0.75, "repful-for": "agree", "repness-test": 2.0412414}, {"tid": 5, "p-test": 2.5, "repness": 1.682352941176471, "n-trials": 15, "n-success": 12, "p-success": 0.7647058823529412, "repful-for": "agree", "repness-test": 1.6796371}, {"tid": 50, "p-test": 2.529822128134704, "repness": 1.636363636363636, "n-trials": 9, "n-success": 8, "p-success": 0.8181818181818182, "repful-for": "agree", "repness-test": 1.5718156}]}, "1": {"0": [{"tid": 32, "p-test": 2.449489742783178, "repness": 1.402597402597403, "n-trials": 5, "n-success": 5, "p-success": 0.8571428571428571, "repful-for": "agree", "repness-test": 1.6926464}, {"tid": 41, "p-test": 2.0, "repness": 1.714285714285714, "n-trials": 3, "n-success": 3, "p-success": 0.8, "repful-for": "agree", "repness-test": 1.8090681}, {"tid": 62, "p-test": 1.4142135623730951, "repness": 2.666666666666667, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "repful-for": "agree", "repness-test": 1.9446898}, {"tid": 28, "p-test": 2.6457513110645907, "repness": 1.166666666666667, "n-trials": 6, "n-success": 6, "p-success": 0.875, "repful-for": "agree", "repness-test": 1.319707}, {"tid": 3, "p-test": -2.121320343559643, "n-agree": 7, "repness": 2.777777777777778, "n-trials": 7, "n-success": 0, "p-success": 0.1111111111111111, "best-agree": true, "repful-for": "disagree", "repness-test": 0.84327406}], "1": [{"tid": 71, "p-test": 0.8164965809277264, "n-agree": 3, "repness": 4.571428571428571, "n-trials": 5, "n-success": 3, "p-success": 0.5714285714285714, "best-agree": true, "repful-for": "agree", "repness-test": 1.9352617}, {"tid": 59, "p-test": 2.0, "repness": 1.4, "n-trials": 3, "n-success": 3, "p-success": 0.8, "repful-for": "agree", "repness-test": 1.4763086}], "2": [{"tid": 59, "p-test": 2.23606797749979, "repness": 1.547619047619048, "n-trials": 4, "n-success": 4, "p-success": 0.8333333333333333, "repful-for": "agree", "repness-test": 1.7179607}, {"tid": 55, "p-test": 2.449489742783178, "repness": 1.285714285714286, "n-trials": 5, "n-success": 5, "p-success": 0.8571428571428571, "repful-for": "agree", "repness-test": 1.4638501}, {"tid": 48, "p-test": 1.341640786499874, "repness": 2, "n-trials": 4, "n-success": 3, "p-success": 0.6666666666666667, "repful-for": "agree", "repness-test": 1.702426}, {"tid": 68, "p-test": 2.0, "repness": 1.333333333333333, "n-trials": 3, "n-success": 3, "p-success": 0.8, "repful-for": "agree", "repness-test": 1.4064217}, {"tid": 3, "p-test": -2.8867513459481287, "n-agree": 11, "repness": 1.615384615384615, "n-trials": 11, "n-success": 0, "p-success": 0.07692307692307691, "best-agree": true, "repful-for": "disagree", "repness-test": 0.37712362}], "3": [{"tid": 41, "p-test": 1.7320508075688772, "repness": 1.5, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "agree", "repness-test": 1.513575}, {"tid": 56, "p-test": 1.7320508075688772, "repness": 1.275, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "agree", "repness-test": 1.2822757}, {"tid": 5, "p-test": -1.0, "n-agree": 3, "repness": 5.2, "n-trials": 3, "n-success": 0, "p-success": 0.2, "best-agree": true, "repful-for": "disagree", "repness-test": 1.538939}, {"tid": 70, "p-test": 2.0, "repness": 9.6, "n-trials": 3, "n-success": 3, "p-success": 0.8, "repful-for": "disagree", "repness-test": 3.3028913}, {"tid": 69, "p-test": 2.0, "repness": 4.4, "n-trials": 3, "n-success": 3, "p-success": 0.8, "repful-for": "disagree", "repness-test": 2.73252}], "4": [{"tid": 51, "p-test": 2.23606797749979, "repness": 2.5, "n-trials": 4, "n-success": 4, "p-success": 0.8333333333333333, "repful-for": "agree", "repness-test": 2.4712634}, {"tid": 33, "p-test": 2.8284271247461903, "repness": 1.777777777777778, "n-trials": 7, "n-success": 7, "p-success": 0.888888888888889, "repful-for": "agree", "repness-test": 2.3841581}, {"tid": 32, "p-test": 2.8284271247461903, "repness": 1.580246913580247, "n-trials": 7, "n-success": 7, "p-success": 0.888888888888889, "repful-for": "agree", "repness-test": 2.0807238}, {"tid": 49, "p-test": 1.341640786499874, "repness": 3.111111111111111, "n-trials": 4, "n-success": 3, "p-success": 0.6666666666666667, "repful-for": "agree", "repness-test": 2.218891}, {"tid": 3, "p-test": -2.121320343559643, "n-agree": 7, "repness": 2.777777777777778, "n-trials": 7, "n-success": 0, "p-success": 0.1111111111111111, "best-agree": true, "repful-for": "disagree", "repness-test": 0.84327406}]}, "2": {"0": [{"tid": 13, "p-test": 1.4142135623730951, "repness": 4.666666666666667, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "repful-for": "agree", "repness-test": 2.108185}, {"tid": 35, "p-test": 1.4142135623730951, "repness": 2.333333333333333, "n-trials": 1, "n-success": 1, "p-success": 0.6666666666666667, "repful-for": "agree", "repness-test": 1.6329932}, {"tid": 3, "p-test": 0.0, "n-agree": 1, "repness": 2.333333333333333, "n-trials": 1, "n-success": 0, "p-success": 0.3333333333333333, "best-agree": true, "repful-for": "disagree", "repness-test": 0.94280905}], "1": [{"tid": 10, "p-test": 2.449489742783178, "n-agree": 5, "repness": 2.571428571428571, "n-trials": 5, "n-success": 5, "p-success": 0.8571428571428571, "best-agree": true, "repful-for": "agree", "repness-test": 1.8516402}, {"tid": 37, "p-test": 2.449489742783178, "repness": 2.571428571428571, "n-trials": 5, "n-success": 5, "p-success": 0.8571428571428571, "repful-for": "agree", "repness-test": 1.8516402}, {"tid": 33, "p-test": 2.449489742783178, "repness": 2.571428571428571, "n-trials": 5, "n-success": 5, "p-success": 0.8571428571428571, "repful-for": "disagree", "repness-test": 1.8516402}]}, "3": {"0": [{"tid": 13, "p-test": 1.7320508075688772, "n-agree": 2, "repness": 3.75, "n-trials": 2, "n-success": 2, "p-success": 0.75, "best-agree": true, "repful-for": "agree", "repness-test": 1.9843135}, {"tid": 5, "p-test": 1.7320508075688772, "repness": 3.75, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "disagree", "repness-test": 1.9843135}, {"tid": 23, "p-test": 1.7320508075688772, "repness": 3.75, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "disagree", "repness-test": 1.9843135}, {"tid": 27, "p-test": 1.7320508075688772, "repness": 3, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "disagree", "repness-test": 1.7320508}, {"tid": 41, "p-test": 1.7320508075688772, "repness": 3, "n-trials": 2, "n-success": 2, "p-success": 0.75, "repful-for": "disagree", "repness-test": 1.7320508}], "1": [{"tid": 71, "p-test": 1.7320508075688772, "n-agree": 2, "repness": 2.25, "n-trials": 2, "n-success": 2, "p-success": 0.75, "best-agree": true, "repful-for": "agree", "repness-test": 1.3693064}]}}, "user-vote-counts": {"0": 72, "1": 2, "2": 4, "3": 18, "4": 18, "5": 18, "6": 17, "7": 18, "8": 18, "9": 18, "10": 20, "11": 65, "12": 39, "13": 65, "14": 24, "15": 48, "16": 60, "17": 29, "18": 9, "19": 15, "20": 29, "21": 12, "22": 14, "23": 30, "24": 30, "25": 31, "26": 8, "27": 31, "28": 31, "29": 33, "30": 33, "31": 42, "32": 42, "33": 42, "34": 5, "35": 3, "36": 47, "37": 1, "38": 15, "39": 14, "40": 1, "41": 17, "42": 5, "43": 48, "44": 28, "45": 48, "46": 1, "47": 50, "48": 3, "49": 48, "50": 28, "51": 51, "52": 53, "53": 52, "54": 6, "55": 22, "56": 54, "57": 55, "58": 9, "59": 64, "60": 60, "61": 64, "62": 17, "63": 40, "64": 62, "65": 28, "66": 62, "67": 11, "68": 63, "69": 62, "70": 12, "71": 24, "72": 64, "73": 21, "74": 13, "75": 64, "76": 65, "77": 11, "78": 65, "79": 19, "80": 2, "81": 15, "82": 65, "83": 5, "84": 9, "85": 65, "86": 65, "87": 65, "88": 37, "89": 48, "90": 65, "91": 65, "92": 65}, "lastVoteTimestamp": 1737467674727, "subgroup-clusters": {"0": [{"id": 0, "center": [-1.8395590724767557, -0.7974827437015634], "members": [15, 19, 22, 29, 41, 44, 48, 54, 72, 75, 79], "parent-id": 0}, {"id": 1, "center": [-1.0474108195357956, 0.42070843654920237], "members": [0, 5, 8, 11, 12, 25, 26, 32, 35, 38, 39, 40, 42, 43, 47, 53, 58, 63, 78, 81], "parent-id": 0}], "1": [{"id": 0, "center": [1.2534666835270112, 1.1358383027206067], "members": [13, 14, 21, 33, 34, 56, 64], "parent-id": 1}, {"id": 1, "center": [0.5403242440559843, -0.06385587086410477], "members": [2, 50, 59, 62, 65, 66, 68, 69, 80], "parent-id": 1}, {"id": 2, "center": [0.20730817526541104, 0.412630845516446], "members": [1, 4, 6, 7, 10, 16, 18, 31, 36, 37, 49, 67, 71, 74], "parent-id": 1}, {"id": 3, "center": [1.5996412982784194, -0.29538541634394433], "members": [20, 57, 70, 77], "parent-id": 1}, {"id": 4, "center": [-0.05613859629785109, 1.5140486656501917], "members": [17, 23, 30, 46, 55, 61, 82], "parent-id": 1}], "2": [{"id": 0, "center": [1.9794073638357446, -0.9435697048412794], "members": [24], "parent-id": 2}, {"id": 1, "center": [4.2685656314013425, -1.4376857078554113], "members": [27, 28, 45, 51, 73], "parent-id": 2}], "3": [{"id": 0, "center": [-2.9754556488703265, -3.15573372713951], "members": [52, 76], "parent-id": 3}, {"id": 1, "center": [0.8329780457004106, -2.1701310962319393], "members": [3, 9, 60], "parent-id": 3}]}, "comment-priorities": {"0": 0.7831078906305026, "1": 49.0, "2": 49.0, "3": 0.8630849522295713, "4": 0.7831078906305026, "5": 1.2125235516319037, "6": 49.0, "7": 0.7831078906305026, "8": 0.7831078906305026, "9": 0.7858625087064816, "10": 0.7393867735854055, "11": 0.24003765839356622, "12": 0.9797919392375388, "13": 1.3261878238466018, "14": 0.7831078906305026, "15": 0.8466981501239679, "16": 0.7831078906305026, "17": 0.8452734127151621, "18": 0.7831078906305026, "19": 0.7831078906305026, "20": 1.1209631974197907, "21": 0.8386757668012835, "22": 0.4785888927047343, "23": 0.9535149500747294, "24": 0.10660714622622411, "25": 0.7559140250012085, "26": 12.529726250088048, "27": 1.7337813111295437, "28": 0.8169857026042904, "29": 0.5745465759110923, "30": 2.379654427761928, "31": 0.9339312095607544, "32": 0.4180989864041549, "33": 2.1108153583588014, "34": 1.5698299459196843, "35": 1.134390641031738, "36": 0.4925764059981745, "37": 0.6216800031714409, "38": 2.9700306934916503, "39": 0.8006843958304772, "40": 0.8506350406424503, "41": 0.28381290202787773, "42": 0.8685671184577414, "43": 0.9034903824748057, "44": 0.5022587886479114, "45": 0.5593463576325884, "46": 0.8279476040223742, "47": 0.4220098390686394, "48": 0.4807080066368163, "49": 0.1522845133804719, "50": 0.20994571018335892, "51": 0.6000980445603505, "52": 0.18813942208431847, "53": 0.776107483075892, "54": 0.8113374758154749, "55": 0.5519818806065255, "56": 0.35027452908252843, "57": 0.04391865433460651, "58": 1.0805841937857192, "59": 0.6391090437621237, "60": 0.5337752459796126, "61": 0.2710403744144893, "62": 0.05604361889625449, "63": 0.7555971423516781, "64": 0.9038444538326186, "65": 0.6417236871741707, "66": 0.7100925683285483, "67": 0.7532405411693863, "68": 0.5025327979234684, "69": 1.4158120885008407, "70": 1.6951196083683404, "71": 0.2121556965921227, "72": 0.499223766962186, "73": 0.5380933179644788}, "group-aware-consensus": {"0": 0.041666666666666664, "1": 0.000051535765821480104, "2": 0.007127019322141274, "3": 0.33421874999999995, "4": 0.041666666666666664, "5": 0.2113726790450928, "6": 0.002087198515769944, "7": 0.041666666666666664, "8": 0.041666666666666664, "9": 0.25425170068027203, "10": 0.4583333333333332, "11": 0.03312629399585921, "12": 0.13149350649350647, "13": 0.08894362342638204, "14": 0.041666666666666664, "15": 0.4950573979591835, "16": 0.041666666666666664, "17": 0.46370967741935476, "18": 0.041666666666666664, "19": 0.041666666666666664, "20": 0.13, "21": 0.4802707373271888, "22": 0.006957328385899814, "23": 0.18803418803418803, "24": 0.004329004329004328, "25": 0.30857142857142855, "26": 0.08333333333333333, "27": 0.12923076923076923, "28": 0.22240802675585283, "29": 0.12041666666666664, "30": 0.013333333333333332, "31": 0.27951388888888884, "32": 0.22134387351778653, "33": 0.050525834476451745, "34": 0.06609195402298851, "35": 0.24739583333333331, "36": 0.31620553359683795, "37": 0.18094405594405594, "38": 0.025, "39": 0.1693958215697346, "40": 0.11515151515151517, "41": 0.012345679012345678, "42": 0.5739130434782609, "43": 0.06820987654320987, "44": 0.13600000000000004, "45": 0.3170278637770897, "46": 0.26794258373205737, "47": 0.10185185185185186, "48": 0.02807017543859649, "49": 0.019999999999999997, "50": 0.07320261437908494, "51": 0.013296398891966758, "52": 0.052380952380952375, "53": 0.27573696145124715, "54": 0.425, "55": 0.3400000000000001, "56": 0.023157894736842106, "57": 0.0032967032967032963, "58": 0.024444444444444442, "59": 0.06778711484593837, "60": 0.32442067736185376, "61": 0.04923076923076923, "62": 0.015384615384615387, "63": 0.38596491228070173, "64": 0.2745098039215686, "65": 0.1730769230769231, "66": 0.2820512820512821, "67": 0.15734265734265737, "68": 0.30303030303030304, "69": 0.08461538461538463, "70": 0.07333333333333335, "71": 0.030769230769230774, "72": 0.12857142857142856, "73": 0.14285714285714285}}	1737467674727	4776	3438	1738330829635
\.


--
-- Data for Name: math_profile; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.math_profile (zid, math_env, data, modified) FROM stdin;
3	prod	{"zid": 3, "opts'": 0.014277, "total": 0.517097, "mod-in": 0.011511, "n-cmts": 9, "n-ptps": 8, "customs": 0.198572, "mod-out": 0.014977, "n-votes": 7, "meta-tids": 0.017773, "recompute": false, "keep-votes": 0.01632, "finish-time": 1738590028570, "raw-rating-mat": 0.214803, "last-vote-timestamp": 1737731284723}	1738590028655
2	prod	{"zid": 2, "opts'": 0.021521, "total": 0.723707, "mod-in": 0.019226, "n-cmts": 74, "n-ptps": 93, "customs": 0.487284, "mod-out": 0.013234, "n-votes": 65, "meta-tids": 0.019817, "recompute": false, "keep-votes": 0.008256, "finish-time": 1738330829083, "last-vote-timestamp": 1737467674727}	1738330829542
5	prod	{"zid": 5, "opts'": 0.018585, "total": 0.575191, "mod-in": 0.013385, "n-cmts": 23, "n-ptps": 21, "customs": 0.135133, "mod-out": 0.018896, "n-votes": 20, "meta-tids": 0.011202, "recompute": false, "keep-votes": 0.013014, "finish-time": 1737841229377, "raw-rating-mat": 0.233729, "last-vote-timestamp": 1736985982519}	1737841229446
11	prod	{"mat": 0.241752, "zid": 11, "tids": 0.007685, "opts'": 0.01546, "total": 0.9202750000000001, "mod-in": 0.011812, "n-cmts": 14, "n-ptps": 16, "customs": 0.149231, "in-conv": 0.104164, "mod-out": 0.009858, "n-votes": 35, "meta-tids": 0.026279, "recompute": false, "keep-votes": 0.008788, "rating-mat": 0.093986, "finish-time": 1739777950503, "raw-rating-mat": 0.173806, "user-vote-counts": 0.053941, "last-vote-timestamp": 1739363141590}	1739777950601
6	prod	{"n": 0.009117, "mat": 0.123871, "zid": 6, "tids": 0.008346, "opts'": 0.014227, "total": 0.742019, "mod-in": 0.010931, "n-cmts": 2, "n-ptps": 1, "customs": 0.027, "in-conv": 0.06928, "n-votes": 1, "consensus": 0.20771, "meta-tids": 0.020969, "recompute": false, "keep-votes": 0.008466, "rating-mat": 0.056005, "finish-time": 1737956428534, "raw-rating-mat": 0.132738, "user-vote-counts": 0.02678, "last-vote-timestamp": 1737096160075}	1737956428577
9	prod	{"zid": 9, "opts'": 0.014838, "total": 1.353759, "mod-in": 0.018114, "n-cmts": 39, "n-ptps": 15, "customs": 0.637484, "mod-out": 0.01576, "n-votes": 92, "meta-tids": 0.017723, "recompute": false, "keep-votes": 0.012113, "finish-time": 1739777950322, "last-vote-timestamp": 1739462776620}	1739777950469
4	prod	{"n": 0.008526, "mat": 0.07963, "pca": 1.842802, "zid": 4, "proj": 0.129282, "tids": 0.008326, "opts'": 0.020809, "total": 4.603626, "mod-in": 0.013976, "n-cmts": 7, "n-ptps": 2, "customs": 0.352341, "in-conv": 0.052056, "mod-out": 0.014327, "n-votes": 14, "consensus": 0.217328, "meta-tids": 0.059031, "proj-nmat": 0.116859, "recompute": false, "bid-to-pid": 0.015429, "keep-votes": 0.036058, "rating-mat": 0.248556, "finish-time": 1737610828716, "base-clusters": 0.519332, "raw-rating-mat": 0.760886, "user-vote-counts": 0.023775, "last-vote-timestamp": 1736756116183, "base-clusters-weights": 0.015929}	1737610828758
12	prod	{"zid": 12, "opts'": 0.022162, "total": 5.784438, "mod-in": 0.024476, "n-cmts": 47, "n-ptps": 61, "customs": 4.672575, "mod-out": 0.01059, "n-votes": 1305, "meta-tids": 0.013335, "recompute": false, "keep-votes": 0.033873, "finish-time": 1739777950935, "last-vote-timestamp": 1739596748921}	1739777951162
\.


--
-- Data for Name: math_ptptstats; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.math_ptptstats (zid, math_env, math_tick, data, modified) FROM stdin;
6	prod	223	{"zid": 6, "ptptstats": {"gid": [0], "pid": [0], "n-votes": [2], "coreness": [1.0], "centricness": [1.0], "extremeness": ["NaN"]}, "lastVoteTimestamp": 1737096160075}	1737956428677
3	prod	485	{"zid": 3, "ptptstats": {"gid": [0, 0, 0, 1, 1, 1, 1, 1], "pid": [1, 4, 6, 0, 2, 3, 5, 7], "n-votes": [2, 5, 7, 1, 3, 4, 6, 7], "coreness": [0.07466215855462088, -1.037026265157083, -0.5836343484537052, 0.3900998879114347, -0.8962536426315013, -0.7213148590475644, -1.39755557507939, -0.3164646644256959], "centricness": [-0.1378783719650607, -1.6408401631370086, -1.7435647448661862, 0.5363799860540852, -1.225644766049013, -1.3557367828580897, -1.3411930637942526, -0.7839031578134712], "extremeness": [-0.6988195496176516, 0.019194300969953892, 0.6796252486476979, -0.5772261925426497, 0.18247802105506905, 0.7987941551152056, -0.6324219419429433, 0.22837595831531807]}, "lastVoteTimestamp": 1737731284723}	1738590028748
4	prod	259	{"zid": 4, "ptptstats": {"gid": [0, 1], "pid": [0, 1], "n-votes": [7, 7], "coreness": [1.0, 1.0], "centricness": [0.0, 0.0], "extremeness": [0.0, 0.0]}, "lastVoteTimestamp": 1736756116183}	1737610828845
5	prod	616	{"zid": 5, "ptptstats": {"gid": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1], "pid": [0, 6, 7, 9, 12, 13, 4, 15, 10, 11, 1, 17, 8, 18, 20, 2, 3, 5, 14, 16], "n-votes": [20, 1, 16, 22, 22, 20, 2, 15, 20, 20, 15, 22, 22, 22, 22, 1, 13, 16, 20, 19], "coreness": [-0.10929075012410805, 0.40228428576995345, 0.8066351505391735, 0.5821839296108311, -2.2134763202250145, 0.4181382125465427, 0.6239001610790305, 0.6463703725054297, 0.2641676003140232, 0.2641676003140232, 0.4213103747845164, 0.32175572946756303, 0.34794131083656155, -0.29264823309552424, 0.7536764183887058, 0.5624666183411323, 0.41606001536705783, -0.2173908223037455, -1.103310862743204, 0.3292043968773891], "centricness": [0.2954456623377901, 0.7132803903714079, 0.5286245353401307, 0.4769540915843935, -2.3638680175299327, 0.31868885710223505, 0.7508577661053905, 0.6862589283389167, -0.08504536634067739, -0.08504536634067739, -0.02400966921008263, -0.05011980615266576, -0.01504159998055199, -0.28153585347501675, 0.7181840261835227, 0.972840178601667, -0.27576786446546175, -0.6171157373915601, -2.6385709923444876, -0.5957098456935934], "extremeness": [-1.0339037946177472, -0.5297165500106353, -0.024664316205511606, -0.11889397600672304, 0.858756133809315, -0.08966348923334908, -0.31481030936634646, -0.2568668168358911, 0.4707324142188387, 0.4707324142188387, 0.5562218978805622, 0.4782689653771525, 0.43696581058621653, -0.2590467300804583, -0.2070432807462123, -0.43706837298805007, -0.5604216411024767, -0.6024831683018913, 1.5015564828585632, -0.33865167345419517]}, "lastVoteTimestamp": 1736985982519}	1737841229546
2	prod	3438	{"zid": 2, "ptptstats": {"gid": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3], "pid": [2, 5, 8, 11, 12, 15, 19, 22, 25, 26, 29, 32, 38, 43, 44, 45, 47, 49, 50, 51, 55, 56, 61, 63, 66, 72, 81, 85, 88, 89, 91, 3, 1, 4, 6, 7, 10, 13, 14, 16, 17, 18, 21, 20, 23, 30, 31, 33, 36, 39, 41, 53, 57, 58, 64, 65, 62, 67, 69, 71, 73, 70, 74, 75, 76, 77, 78, 79, 84, 87, 90, 92, 24, 27, 28, 52, 59, 82, 0, 9, 60, 68, 86], "n-votes": [4, 18, 18, 65, 39, 48, 15, 14, 31, 8, 33, 42, 15, 48, 28, 48, 50, 48, 28, 51, 22, 54, 64, 40, 62, 64, 15, 65, 37, 48, 65, 18, 2, 18, 17, 18, 20, 65, 24, 60, 29, 9, 12, 29, 30, 33, 42, 42, 47, 14, 17, 52, 55, 9, 62, 28, 17, 11, 62, 24, 21, 12, 13, 64, 65, 11, 65, 19, 9, 65, 65, 65, 30, 31, 31, 53, 64, 65, 72, 18, 60, 63, 65], "coreness": [-0.16293018260590442, 0.17408792714791232, 0.15205372666787786, -0.4893473998519646, 0.749172536817989, -0.48274246348010497, 0.5659616403727834, -0.44348739173381824, -0.34452688397911335, 0.45488659303040635, 0.24845537138830442, 0.6937322175287202, 0.22389110801676737, -0.00967736559916399, 0.3412106881437539, 0.2401991775429282, 0.5397371293982207, 0.9414923799219379, 0.8928378129443303, -0.140866844692906, 0.5734229142470066, 0.09603219814078445, 0.2136268158623983, 0.14303140909947276, 0.4842859039711078, 0.435169243969865, 0.6148794939481929, -0.07135656174172755, 0.038521246892677996, -1.505743443490228, 0.10341338612334261, 0.857319986539799, -0.0015805654967269156, 0.46143303877237696, 0.8558169922504093, 0.6669739541798677, 0.3055936898832904, -0.4906857642827307, -0.12621953262997532, 0.8954487811696065, 0.2616644957791425, 0.776513401047629, -0.3617593329255959, 0.6020081140384733, -0.08012460658978782, -0.1022623355827812, 0.06740149427817177, -0.000016870520341383255, 0.22616655293495913, 0.9498213789342502, 0.5480297753409504, -0.47327423880510255, 0.8926711602612802, 0.31547366921022646, 0.05106524256264633, 0.14583816266772953, 0.0953085337483437, 0.5360257962800334, -0.3947408449965464, 0.2644773993181858, 0.07498037062119656, 0.1752004810346388, 0.5090174055313119, 0.46612250578637326, 0.036337753973107234, 0.25891952137371443, -0.6850712771976231, 0.23859929028489646, 0.40877532797357463, -0.6289548976195221, 0.6282316862959144, -0.47630820303643584, -0.9515656461185555, 0.8913728949954216, -0.8626009722278862, 0.48328980490892615, -0.04722276526665792, 0.28906655557267225, -0.23643384440639914, -0.7769719636929648, -1.066346054683092, -1.150562544573695, -1.8868080612955462], "centricness": [0.5358525578952549, 0.39438120364924045, 0.347016441981554, -1.0492635659758176, -0.10557954642672285, -1.354553149444706, -0.5640540532975682, -1.5493345645950427, -0.6365361454565819, 0.18634423203504447, -1.1056896948096773, -0.16074744102884186, 0.35254704727833563, -0.9471923674879879, 0.03126765064417791, -0.4139433760365381, -0.7971168058640989, -0.3034170917615937, -0.2703232575363208, -0.6123248942126938, -0.2616580377550157, -1.1092363698231846, -0.7041805353842396, -0.5344835692390288, 0.12656036217199573, -0.08617490218700241, -0.6310198358872612, -1.0104148228811685, -0.39460971381538235, -2.6388918445130374, -1.014687171543378, 0.35115834462408657, 0.5706085279678655, 0.42337118617281366, 0.3879500589484032, 0.4159003686247079, 0.563789831220213, -1.2282015418849261, -0.8716880760542298, 0.28280866221901946, -0.07020983320214524, 0.4726658373533157, -0.6500615267466094, -0.1455414452920023, -0.4635063736888787, -0.34126230810215286, 0.45625124400239647, -0.7495692695389311, -0.5074911390630548, 0.2983685166479797, 0.6982195283258537, -0.7511441665607166, 0.3118906882662962, 0.6261458171187313, -0.28611491463828265, -0.5827081453101914, -0.1601649671759413, 0.14460689538304183, -0.9888618743440818, 0.23284138409198318, -0.6517125486436062, 0.42935512783460583, -0.011507899909070307, 0.7709951175242586, 0.7774141403823651, 0.41125263998028694, -1.1117689964802904, 0.5811431812289736, 0.7936201554916961, -0.6310252575544184, 0.3538972049604604, -0.7620402936726569, -1.1755865524497233, -3.1824996288059415, -4.527183964475848, -3.613565159551598, -3.0366453482305804, -3.4169516025157893, -0.5419135536472968, -1.497973099674506, -3.4737136522534415, -2.0858557695400517, -3.4206939121028945], "extremeness": [-1.0968983639161487, -0.7935430418028278, -0.7851502619015241, 0.05443189814085859, -0.24904033007675244, 0.5580701818290681, 0.1565390734183498, 0.9531532835606509, -0.35570490491915624, -0.5423852457046979, 0.7514346530975466, -0.21428111718340412, -0.7447299427110011, 0.34640318228551203, -0.49086025030162717, -0.15210346630117236, 0.4371114622316698, -0.05110981590535238, -0.08553399323667818, -0.19785842438108092, -0.1565804435872739, 0.6637863306054193, 0.16686709078228196, -0.07888541689146704, -0.49363057510016567, -0.3593069447537549, 0.2503321447426855, 0.3913934803345628, -0.3003234626275636, 1.8936728052819818, 0.5247304149910518, -0.10856894131552543, -0.9206823744534229, -0.34711763441153237, -0.13975117753810248, -0.2220333571414222, -0.5697998246526967, 1.4512250415757606, 1.1129785042546627, -0.040109733734188996, 0.02434210859495972, -0.22345254446660545, 0.20289478007322537, 0.3930676924480636, 0.2740662806102022, 0.013548257965571686, -0.7577106334450103, 0.9971731286203158, 0.7392736498880217, -0.04920943923414958, -0.4507789263698539, 0.22131204787421532, -0.067719645503155, -0.5943668848012885, 0.12650666267113625, 0.8069781334093793, -0.024070486476206687, -0.03145674766751844, 0.963414984692189, -0.34372617417513623, 0.8716584651787296, -0.6115820442519336, 0.14546708825642343, -0.5302812705692881, -0.9608502579459339, -0.5102984330727209, 0.7035715821356023, -0.6446355621972563, -0.5798116224372138, -0.370891211346584, -0.1894195326177688, 0.24084605157606254, -1.9356003549357164, 0.08507717231473835, 1.256576425999811, 0.5166852830885859, -0.19363184208101775, 0.27089331561359187, -1.1789350547209483, -0.7614774613503825, 1.6051477505959082, -0.4221776140240575, 0.7574423794994798]}, "lastVoteTimestamp": 1737467674727}	1738330829732
9	prod	259	{"zid": 9, "ptptstats": {"gid": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], "pid": [0, 2, 3, 4, 5, 1, 7, 6, 8, 10, 11, 13, 12, 14, 9], "n-votes": [1, 2, 2, 3, 19, 1, 1, 10, 10, 2, 9, 9, 10, 10, 2], "coreness": [0.9368878026631152, 0.9368878026631152, 0.9368878026631152, 0.9368878026631152, 0.9368878026631152, 0.9368878026631152, 0.9368878026631152, 0.9368878026631152, -2.7219311208477786, 0.9331553097679098, 0.02098480092389754, 0.20390988656194597, 0.033656444504183036, 0.033656444504183036, 1.0], "centricness": [0.9938013703550008, 0.9938013703550008, 0.9938013703550008, 0.9938013703550008, 0.9938013703550008, 0.9938013703550008, 0.9938013703550008, 0.9938013703550008, -2.7267601188953288, 0.8717179577975198, 0.017266855273717185, 0.1861013624000052, 0.02996467936508218, 0.02996467936508218, 0.03393853091142107], "extremeness": [-0.0630853507303754, -0.0630853507303754, -0.0630853507303754, -0.0630853507303754, -0.0630853507303754, -0.0630853507303754, -0.0630853507303754, -0.0630853507303754, 0.22613133186930998, 0.05236234783442037, 0.018347149861784554, 0.17324924848627454, 0.01729636389560684, 0.01729636389560684, 0.0]}, "lastVoteTimestamp": 1739462776620}	1739777950611
11	prod	185	{"zid": 11, "ptptstats": {"gid": [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1], "pid": [1, 3, 4, 0, 6, 8, 7, 2, 5, 9, 11, 12, 13, 14, 10], "n-votes": [2, 2, 2, 2, 2, 2, 1, 2, 2, 1, 7, 3, 1, 1, 4], "coreness": [-2.2248037026895138, -2.8188288567950077, -2.735627721874271, 0.48117039698672814, 0.48117039698672814, 0.48117039698672814, 0.3132116525688051, -1.5827668998519564, -0.9202331648743323, 0.25033807562444577, 0.25033807562444577, 0.25033807562444577, 0.25033807562444577, 0.25033807562444577, -0.3343744477583086], "centricness": [-3.121650040738581, -2.73596432975828, -2.785828789171409, 0.6192578995269316, 0.6192578995269316, 0.6192578995269316, 0.5896064132643892, -2.36183432605251, -1.3726946188933762, 0.9562812423150415, 0.9562812423150415, 0.9562812423150415, 0.9562812423150415, 0.9562812423150415, -0.7060270700315001], "extremeness": [3.2124279383658516, -0.7977398947084786, -0.2398979371238794, -0.5188189159161789, -0.5188189159161789, -0.5188189159161789, -0.6183333587849578, 2.5663906203367954, 0.8491001189580758, -0.7491459022961288, -0.7491459022961288, -0.7491459022961288, -0.7491459022961288, -0.7491459022961288, 0.3302387721857723]}, "lastVoteTimestamp": 1739363141590}	1739777950710
12	prod	1259	{"zid": 12, "ptptstats": {"gid": [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3], "pid": [29, 17, 31, 34, 57, 4, 6, 30, 2, 22, 3, 12, 13, 18, 10, 38, 16, 44, 35, 53, 54, 56, 21, 9, 32, 11, 33, 36, 40, 8, 43, 1, 39, 60, 0, 26, 20, 5, 7, 14, 27, 45, 41, 49, 51, 52, 15, 59], "n-votes": [30, 34, 33, 33, 13, 21, 3, 33, 35, 32, 35, 31, 19, 35, 33, 17, 33, 21, 33, 30, 23, 27, 34, 19, 12, 33, 14, 27, 27, 30, 21, 1, 27, 40, 42, 11, 30, 35, 21, 23, 33, 21, 32, 32, 21, 10, 33, 24], "coreness": [-0.37606608251356133, -0.18377445663494174, -1.4241081029287366, -0.013683593766464108, -0.39293152918266494, 0.03465102076202786, 0.5058216122435022, -0.7715511189882955, 0.18142345736978516, 0.5764781783457826, 0.13704960227600804, 0.5636952725405877, 0.009765351829071012, -0.08329443920242108, -0.48766788544024364, 0.2760957907728705, 0.18679987256173314, 0.16764363400274163, 0.42531486230076077, 0.20313606836282894, 0.1075727891540339, 0.3467721377273403, 0.11615081812050021, 0.40012605400723633, -0.3292496215337175, -1.0813172206659822, 0.20442974767552535, 0.18901065946234497, 0.44708200520297403, 0.12405876950156514, -0.05571833131696469, -0.350950226580131, 0.26600084153297954, -0.04288116749320414, 0.46751240454410914, -0.04162894350947588, -1.0266276972801194, 0.625165726725516, 0.7372170051085518, 0.42206114826032537, -0.13689360270423245, 0.400938059063186, -0.20309036198083708, -0.07756178045236517, 0.31294498036990537, 0.6954531568111328, -0.5700880263430053, 0.549384020179285], "centricness": [-1.0876240981257883, -0.7458600910805757, -3.8140791528815665, -1.0532197577296083, -2.975359755038755, -0.1753787156883373, -0.17423939649579379, -2.030217141389474, 0.2194824061291063, -0.7134154276051228, -0.39001518356539444, -0.6765502778247654, -1.1746782030293383, 0.6196776003821014, -1.2574182672325085, -0.44719016566009384, 0.35951972240450303, -0.4387241292381532, -0.6759167679320033, 0.42265683709482493, -1.2543605231389776, -0.8968031946960637, -0.02990633762735806, 0.12859655785818147, -1.3020389622107356, -2.4342896765156223, -0.9478371531600793, 0.32623049653045877, -0.9818203042971283, -1.3209546744478162, 0.583649369936388, 0.8862007439432282, 0.22862188670719108, -1.4434803248021213, -0.2945637341681162, -1.1714944973780193, -1.6572564423641132, -0.3705124002017892, -0.30017292601853507, -0.4991035290484138, 0.23907919577481507, -0.31643818384755296, 0.14737729116244758, -1.2385611475530598, -1.0618536741833813, -0.7720905772327602, -1.728999728176695, -0.975241063758703], "extremeness": [-0.9424407960949387, -1.0898913697366517, 1.7318683375672042, -0.812003071673164, 1.1124668999375489, -0.52298911581357, -0.2735976974946429, 1.5136938459293137, -0.7088382601682517, 0.31684960641025384, -0.2543628056025793, 0.2673496666456458, 0.6780940977846786, -1.0612660443971926, 0.36285342739906423, -0.114973758073568, -0.7780915290343791, -0.18531648910496212, 0.21564261108076632, -0.7965089366454646, 0.8736195319396807, 0.4678418491452083, -0.625821305173454, -0.5843086309207144, 0.49971551388476376, 1.8595052735814535, 0.37123003148186723, -0.7930297221351467, 0.5306532965098447, 0.8758488546720591, -1.0481919261360242, -1.349534531892233, -0.7030660444965183, 0.9669991906241028, -0.30576619439755953, 0.42815452402302084, 0.20627572000880212, -0.19248258139641441, -0.23065643348179898, -0.13493276291364498, -0.9963456530792038, -0.31174039141096066, -0.9986196272690216, 0.5001740794339067, 0.47694991115564855, 0.23702377133542396, 0.8714709265887196, 0.4504947114030863]}, "lastVoteTimestamp": 1739596748921}	1739777951267
\.


--
-- Data for Name: math_report_correlationmatrix; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.math_report_correlationmatrix (rid, math_env, data, math_tick, modified) FROM stdin;
\.


--
-- Data for Name: math_ticks; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.math_ticks (zid, math_tick, caching_tick, math_env, modified) FROM stdin;
5	616	0	prod	1737841229469
3	485	0	prod	1738590028682
6	223	0	prod	1737956428617
4	259	0	prod	1737610828779
9	259	0	prod	1739777950489
11	185	0	prod	1739777950626
12	1259	0	prod	1739777951183
2	3438	0	prod	1738330829578
\.


--
-- Data for Name: metrics; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.metrics (uid, type, dur, hashedpc, created) FROM stdin;
\.


--
-- Data for Name: notification_tasks; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.notification_tasks (zid, modified) FROM stdin;
\.


--
-- Data for Name: oinvites; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.oinvites (oinvite, note, created) FROM stdin;
\.


--
-- Data for Name: page_ids; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.page_ids (site_id, page_id, zid) FROM stdin;
\.


--
-- Data for Name: participant_locations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.participant_locations (zid, uid, pid, lat, lng, created, source) FROM stdin;
\.


--
-- Data for Name: participant_metadata_answers; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.participant_metadata_answers (pmaid, pmqid, zid, value, alive, created) FROM stdin;
\.


--
-- Data for Name: participant_metadata_choices; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.participant_metadata_choices (zid, pid, pmqid, pmaid, alive, created) FROM stdin;
\.


--
-- Data for Name: participant_metadata_questions; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.participant_metadata_questions (pmqid, zid, key, alive, created) FROM stdin;
\.


--
-- Data for Name: participants; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.participants (pid, uid, zid, vote_count, last_interaction, subscribed, last_notified, nsli, mod, created) FROM stdin;
28	36	2	31	1736657950894	0	0	0	0	1736657847066
24	32	2	30	1736657428400	0	0	0	0	1736657211864
29	37	2	33	1736658190587	0	0	0	0	1736657926202
7	15	2	18	1736654621674	0	0	0	0	1736654561433
5	13	2	18	1736654473863	0	0	0	0	1736654381050
22	30	2	14	1736657273387	0	0	0	0	1736657185863
5	7	3	6	1736651811307	0	0	0	0	1736651807283
12	20	2	39	1736743875864	0	0	0	0	1736655204361
39	47	2	14	1736696262792	0	0	0	0	1736696208673
20	28	2	29	1736657337633	0	0	0	0	1736657159887
18	26	2	9	1736657067100	0	0	0	0	1736657017435
53	61	2	52	1736702799966	0	0	0	0	1736702311364
30	38	2	33	1736658275118	0	0	0	0	1736658160258
40	48	2	1	1736699354533	0	0	0	0	1736699354402
46	54	2	1	1736700869862	0	0	0	0	1736700869732
6	8	3	7	1736651840834	0	0	0	0	1736651834000
62	70	2	17	1736734922156	0	0	0	0	1736734137893
4	12	2	18	1736653718824	0	0	0	0	1736653654766
3	11	2	18	1736653726447	0	0	0	0	1736653653731
2	10	2	4	1736652378114	0	0	0	0	1736652375070
0	1	3	1	1736648104726	0	0	0	0	1736648104570
1	3	3	2	1736648231493	0	0	0	0	1736648226828
56	64	2	54	1736708882167	0	0	0	0	1736708201402
38	46	2	15	1736690550116	0	0	0	0	1736690333397
10	18	2	20	1736655204818	0	0	0	0	1736654854122
69	77	2	62	1736735977590	0	0	0	0	1736735704384
9	17	2	18	1736654917593	0	0	0	0	1736654747976
0	2	2	72	1736736364917	0	0	0	0	1736645112945
33	41	2	42	1736666211822	0	0	0	0	1736666020897
17	25	2	29	1736657130103	0	0	0	0	1736656787607
16	24	2	60	1736724745008	0	0	0	0	1736656341071
8	16	2	18	1736654686314	0	0	0	0	1736654654172
2	4	3	3	1736651757290	0	0	0	0	1736651755466
25	33	2	31	1736657707750	0	0	0	0	1736657594923
6	14	2	17	1736654574980	0	0	0	0	1736654517929
21	29	2	12	1736657218004	0	0	0	0	1736657170174
3	5	3	4	1736651774118	0	0	0	0	1736651769873
32	40	2	42	1736664737559	0	0	0	0	1736664442704
19	27	2	15	1736657168511	0	0	0	0	1736657028355
14	22	2	24	1736655672049	0	0	0	0	1736655354520
4	6	3	5	1736651793059	0	0	0	0	1736651789108
36	44	2	47	1736678370172	0	0	0	0	1736678082197
44	52	2	28	1736700206570	0	0	0	0	1736699984873
37	45	2	1	1736679614160	0	0	0	0	1736679613996
31	39	2	42	1736663834335	0	0	0	0	1736663252739
55	63	2	22	1736703712596	0	0	0	0	1736703594989
27	35	2	31	1736658030014	0	0	0	0	1736657793184
23	31	2	30	1736657396526	0	0	0	0	1736657209887
73	81	2	21	1736737079799	0	0	0	0	1736736989017
47	55	2	50	1736701918770	0	0	0	0	1736701402282
1	9	2	2	1736703786984	0	0	0	0	1736652321556
48	56	2	3	1736701497423	0	0	0	0	1736701484316
26	34	2	8	1736657817242	0	0	0	0	1736657746091
41	49	2	17	1736699726569	0	0	0	0	1736699599422
63	71	2	40	1736734620525	0	0	0	0	1736734372929
35	43	2	3	1736673047339	0	0	0	0	1736673036854
45	53	2	48	1736701128804	0	0	0	0	1736700557262
52	60	2	53	1736704545320	0	0	0	0	1736702009816
58	66	2	9	1736717931322	0	0	0	0	1736717867267
57	65	2	55	1736709173233	0	0	0	0	1736708752149
15	23	2	48	1736699970084	0	0	0	0	1736656290995
54	62	2	6	1736703211532	0	0	0	0	1736703188173
49	57	2	48	1736701773036	0	0	0	0	1736701502200
43	51	2	48	1736700055050	0	0	0	0	1736699797659
42	50	2	5	1736699783607	0	0	0	0	1736699755044
51	59	2	51	1736702429883	0	0	0	0	1736701789102
66	74	2	62	1736735694251	0	0	0	0	1736735101033
67	75	2	11	1736735678386	0	0	0	0	1736735626115
60	68	2	60	1736732883816	0	0	0	0	1736732431761
65	73	2	28	1736734813637	0	0	0	0	1736734566277
64	72	2	63	1736734841960	0	0	0	0	1736734393746
50	58	2	28	1736734785073	0	0	0	0	1736701670339
70	78	2	12	1736737066630	0	0	0	0	1736736427813
68	76	2	63	1736736258650	0	0	0	0	1736735670828
71	79	2	24	1736737732803	0	0	0	0	1736736463992
72	80	2	64	1736738738407	0	0	0	0	1736736532719
74	82	2	13	1736739040675	0	0	0	0	1736738953569
61	69	2	64	1736738757305	0	0	0	0	1736734107421
75	83	2	64	1736740538875	0	0	0	0	1736740161598
59	67	2	64	1736744828103	0	0	0	0	1736719950177
76	84	2	65	1736743015848	0	0	0	0	1736742667024
77	85	2	11	1736744340672	0	0	0	0	1736744241297
0	2	4	7	1736756098157	0	0	0	0	1736754940877
1	86	4	7	1736756116286	0	0	0	0	1736756067758
13	21	2	65	1737062909935	0	0	0	0	1736655285260
11	19	2	65	1736879545729	0	0	0	0	1736654927205
34	42	2	5	1737406068752	0	0	0	0	1736669336085
2	47	5	1	1736794422943	0	0	0	0	1736794422818
0	2	5	20	1736802184965	0	0	0	0	1736789772081
0	122	11	2	1739296903457	0	0	0	0	1739296894609
1	87	5	15	1736794607729	0	0	0	0	1736794349418
4	89	5	2	1736794601872	0	0	0	0	1736794490283
19	102	5	1	1736814214477	0	0	0	0	1736814214354
88	114	2	38	1736996051924	0	0	0	0	1736993941559
12	96	5	22	1736985982622	0	0	0	0	1736801499011
9	180	9	2	1739454040043	0	0	0	0	1739453994088
90	116	2	65	1737041950103	0	0	0	0	1737041230345
0	114	6	2	1737096160179	1	0	0	0	1737084575555
11	133	11	7	1739351744910	0	0	0	0	1739308727056
14	235	9	10	1739462776723	0	0	0	0	1739462747996
82	108	2	65	1736881040002	0	0	0	0	1736880375723
81	107	2	15	1736879961525	0	0	0	0	1736879876170
14	98	5	20	1736803951378	0	0	0	0	1736803832849
14	173	11	1	1739362870396	0	0	0	0	1739362870263
10	182	9	2	1739454378334	0	0	0	0	1739454374647
48	231	12	1	1739461875505	0	0	0	0	1739461875388
3	175	9	2	1739371792657	0	0	0	0	1739371790528
10	23	5	20	1736797186909	0	0	0	0	1736797077193
92	118	2	65	1737467674835	0	0	0	0	1737467377046
27	207	12	33	1739466338243	0	0	0	0	1739454891750
17	100	5	22	1736809833079	0	0	0	0	1736809741534
14	194	12	23	1739462934853	0	0	0	0	1739454686951
37	219	12	2	1739459707526	0	0	0	0	1739459703354
46	229	12	2	1739461177658	0	0	0	0	1739461093146
84	110	2	9	1736880757404	0	0	0	0	1736880731779
15	99	5	15	1736804721871	0	0	0	0	1736804657916
87	113	2	65	1736917362043	0	0	0	0	1736916054113
19	199	12	3	1739454789229	0	0	0	0	1739454706647
78	104	2	65	1736827957783	0	0	0	0	1736826738922
50	233	12	3	1739461921995	0	0	0	0	1739461906749
6	186	12	3	1739454576105	0	0	0	0	1739454565975
80	106	2	2	1736879804460	0	0	0	0	1736879802977
7	119	3	7	1737731284826	0	0	0	0	1737730363480
1	124	11	2	1739296923226	0	0	0	0	1739296921432
32	213	12	12	1739456583773	0	0	0	0	1739456444963
3	126	11	2	1739296943482	0	0	0	0	1739296942906
35	217	12	33	1739466347153	0	0	0	0	1739459642299
45	227	12	21	1739461239055	0	0	0	0	1739460441680
5	128	11	2	1739296960396	0	0	0	0	1739296959217
12	215	9	10	1739457840248	0	0	0	0	1739457806417
7	130	11	1	1739296989930	0	0	0	0	1739296989792
43	225	12	21	1739459976565	0	0	0	0	1739459904671
9	119	11	1	1739302658436	0	0	0	0	1739302658057
85	111	2	65	1736881928924	0	0	0	0	1736881628276
0	170	9	1	1739317205413	0	0	0	0	1739317205047
59	243	12	24	1739466147563	0	0	0	0	1739465962083
13	171	11	1	1739318781305	0	0	0	0	1739318781064
39	221	12	27	1739464188777	0	0	0	0	1739459796795
5	122	9	19	1739384339319	0	0	0	0	1739378649927
11	209	9	9	1739457076946	0	0	0	0	1739455389390
7	178	9	1	1739384417740	0	0	0	0	1739384417618
9	189	12	19	1739459087954	0	0	0	0	1739454624480
22	202	12	32	1739465972179	0	0	0	0	1739454726513
57	241	12	13	1739464351966	0	0	0	0	1739464168426
4	184	12	21	1739459598034	0	0	0	0	1739454485145
30	211	12	33	1739466290032	0	0	0	0	1739455626199
55	239	12	3	1739463407507	0	0	0	0	1739463386548
11	191	12	33	1739466522854	0	0	0	0	1739454656450
20	200	12	30	1739465107171	0	0	0	0	1739454718139
41	223	12	32	1739465957360	0	0	0	0	1739459827585
17	197	12	34	1739468134668	0	0	0	0	1739454699295
53	237	12	30	1739465245868	0	0	0	0	1739462850006
0	122	12	42	1739468201056	0	0	0	0	1739381900419
36	218	12	27	1739464127973	0	0	0	0	1739459655468
16	42	5	19	1736807278511	0	0	0	0	1736807068220
2	125	11	2	1739296934274	0	0	0	0	1739296933279
83	109	2	5	1736880487141	0	0	0	0	1736880472158
9	94	5	22	1736808503840	0	0	0	0	1736795587114
8	179	9	10	1739457711436	0	0	0	0	1739453811238
33	214	12	14	1739457894731	0	0	0	0	1739457739226
11	95	5	20	1736797908639	0	0	0	0	1736797828989
6	177	9	10	1739381490801	0	0	0	0	1739381298448
3	88	5	13	1736794542626	0	0	0	0	1736794459152
8	93	5	22	1736809839193	0	0	0	0	1736795556741
4	127	11	2	1739296950952	0	0	0	0	1739296950355
23	203	12	3	1739454751975	0	0	0	0	1739454734189
1	137	12	1	1739382153288	0	0	0	0	1739382153153
6	129	11	2	1739296970227	0	0	0	0	1739296969748
18	101	5	22	1736814202669	0	0	0	0	1736814068071
8	131	11	2	1739296991017	0	0	0	0	1739296989984
79	105	2	19	1736879905117	0	0	0	0	1736879781075
7	92	5	16	1736795629572	0	0	0	0	1736795556240
89	115	2	48	1737002603049	0	0	0	0	1737002015148
24	204	12	3	1739454762233	0	0	0	0	1739454734752
13	228	9	9	1739460728920	0	0	0	0	1739460658379
56	240	12	27	1739464353369	0	0	0	0	1739463945503
10	132	11	4	1739308314067	0	0	0	0	1739308139088
15	195	12	33	1739466030132	0	0	0	0	1739454690567
5	90	5	16	1736794973468	0	0	0	0	1736794871129
6	91	5	1	1736795450976	0	0	0	0	1736795450845
12	135	11	3	1739316275392	0	0	0	0	1739316195964
1	137	9	1	1739317231909	0	0	0	0	1739317231711
25	205	12	3	1739454848476	0	0	0	0	1739454774973
13	97	5	20	1736802941739	0	0	0	0	1736802790674
2	172	9	2	1739345831946	0	0	0	0	1739345825843
38	220	12	17	1739459910221	0	0	0	0	1739459790996
15	174	11	1	1739363141695	0	0	0	0	1739363141566
86	112	2	65	1736882633046	0	0	0	0	1736882248299
31	212	12	33	1739466301303	0	0	0	0	1739456152929
4	176	9	4	1739378808067	0	0	0	0	1739378620301
20	103	5	22	1736814656796	0	0	0	0	1736814454247
91	117	2	65	1737265536136	0	0	0	0	1737265238146
16	196	12	33	1739466081101	0	0	0	0	1739454692401
60	245	12	40	1739596749026	0	0	0	0	1739595941191
44	226	12	21	1739460280585	0	0	0	0	1739460044140
18	198	12	39	1739480968780	0	0	0	0	1739454704513
21	201	12	34	1739534278939	0	0	0	0	1739454720104
54	238	12	23	1739463061948	0	0	0	0	1739462896449
58	242	12	3	1739464201307	0	0	0	0	1739464196996
51	234	12	21	1739462387676	0	0	0	0	1739462267980
7	187	12	21	1739459916639	0	0	0	0	1739454612382
47	230	12	4	1739461841374	0	0	0	0	1739461833742
5	185	12	35	1739468240029	0	0	0	0	1739454492651
28	208	12	5	1739455035233	0	0	0	0	1739454999012
42	224	12	2	1739459894830	0	0	0	0	1739459887801
34	216	12	34	1739466254437	0	0	0	0	1739458134527
12	192	12	31	1739465718502	0	0	0	0	1739454663340
26	206	12	11	1739456260963	0	0	0	0	1739454852980
13	193	12	19	1739459802997	0	0	0	0	1739454664836
10	190	12	33	1739466548952	0	0	0	0	1739454654367
2	181	12	35	1739468310708	0	0	0	0	1739454339159
49	232	12	32	1739465967186	0	0	0	0	1739461878727
3	183	12	40	1739468503219	0	0	0	0	1739454392201
52	236	12	10	1739462872905	0	0	0	0	1739462818962
29	210	12	30	1739465232862	0	0	0	0	1739455585254
8	188	12	31	1739465465863	0	0	0	0	1739454612437
40	222	12	27	1739464254980	0	0	0	0	1739459805360
\.


--
-- Data for Name: participants_extended; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.participants_extended (uid, zid, referrer, parent_url, created, modified, subscribe_email, show_translation_activated, permanent_cookie, origin) FROM stdin;
2	2	\N	\N	1736645112931	1736645112931	\N	\N	\N	\N
1	3	\N	\N	1736648104557	1736648104557	\N	\N	\N	\N
3	3	\N	\N	1736648226818	1736648226818	\N	\N	\N	\N
4	3	\N	\N	1736651755456	1736651755456	\N	\N	\N	\N
5	3	\N	\N	1736651769870	1736651769870	\N	\N	\N	\N
6	3	\N	\N	1736651789098	1736651789098	\N	\N	\N	\N
7	3	\N	\N	1736651807280	1736651807280	\N	\N	\N	\N
8	3	\N	\N	1736651833993	1736651833993	\N	\N	\N	\N
10	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736652375067	1736652375080	\N	\N	iszMAHSecqoNySiyEbhH	https://poliscommunity.crown-shy.com
11	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736653653728	1736653653740	\N	\N	fcjkxzeD0HqHcT1QkIbZ	https://poliscommunity.crown-shy.com
12	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736653654764	1736653654772	\N	\N	rigwvG8w1jW5ebt2J58K	https://poliscommunity.crown-shy.com
13	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736654381045	1736654381061	\N	\N	dWSzDRD2iCok1HopaybV	https://poliscommunity.crown-shy.com
14	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736654517926	1736654517938	\N	\N	TSmBBauPRUlZupOVoE8b	https://poliscommunity.crown-shy.com
15	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736654561429	1736654561441	\N	\N	IoAevZxAJypsKqorBySt	https://poliscommunity.crown-shy.com
16	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736654654169	1736654654181	\N	\N	xKrGJEvFhddLyOX3X343	https://poliscommunity.crown-shy.com
17	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736654747973	1736654747985	\N	\N	6tmFv8CjFAWKtZax0KPV	https://poliscommunity.crown-shy.com
18	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736654854119	1736654854129	\N	\N	IoCDVLRpmXVM40sph41a	https://poliscommunity.crown-shy.com
19	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736654927202	1736654927212	\N	\N	7BqAReWM1pu3i494IX41	https://poliscommunity.crown-shy.com
20	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736655204357	1736655204372	\N	\N	kiGbYzmcIdaR04F0esx6	https://poliscommunity.crown-shy.com
21	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736655285256	1736655285269	\N	\N	N1SUnG73m6Mk5lYGrirj	https://poliscommunity.crown-shy.com
22	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736655354517	1736655354527	\N	\N	m705097wWFtN4ptakJDp	https://poliscommunity.crown-shy.com
23	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736656290992	1736656291006	\N	\N	4o0GqdNNBToizmzCeqTH	https://poliscommunity.crown-shy.com
24	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736656341057	1736656341091	\N	\N	fLceKdcOSdDF7nzxdQM5	https://poliscommunity.crown-shy.com
25	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736656787604	1736656787613	\N	\N	mb2LlPh3dH1KTW6aqQoj	https://poliscommunity.crown-shy.com
26	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736657017431	1736657017444	\N	\N	9oR8ulWlk52FvXqHeiIS	https://poliscommunity.crown-shy.com
27	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736657028353	1736657028358	\N	\N	sR5f6KMdBRnmp2Ck6JXn	https://poliscommunity.crown-shy.com
28	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736657159883	1736657159895	\N	\N	75nNm7TqXpm75PLhANoJ	https://poliscommunity.crown-shy.com
29	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736657170171	1736657170181	\N	\N	2MQ62MeNzfC8yMyJAeqU	https://poliscommunity.crown-shy.com
30	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736657185860	1736657185872	\N	\N	6mjzsRPcNqTo4Nom9vG3	https://poliscommunity.crown-shy.com
31	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736657209884	1736657209894	\N	\N	0m6g27MaMaPBjsGcJtIO	https://poliscommunity.crown-shy.com
32	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736657211861	1736657211868	\N	\N	jgraALo4Fi41RaH9j8X8	https://poliscommunity.crown-shy.com
33	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736657594920	1736657594930	\N	\N	DeEfgEBe8CpBOLwat7nt	https://poliscommunity.crown-shy.com
34	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736657746088	1736657746097	\N	\N	MqL7D3wmbTFselqNgvRo	https://poliscommunity.crown-shy.com
35	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736657793181	1736657793190	\N	\N	1OJKqnfL1Uw9aUINX0Fy	https://poliscommunity.crown-shy.com
36	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736657847062	1736657847073	\N	\N	F9nKU5m8kR3Gp6cFt54y	https://poliscommunity.crown-shy.com
37	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736657926199	1736657926207	\N	\N	yBrvUElUkLKHxFzDbGr5	https://poliscommunity.crown-shy.com
38	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736658160254	1736658160263	\N	\N	gxFYjxB34Ah6rmiVi8rt	https://poliscommunity.crown-shy.com
39	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736663252735	1736663252749	\N	\N	6ytSTBivxJH8G2WLZiam	https://poliscommunity.crown-shy.com
40	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736664442700	1736664442713	\N	\N	rjOWMECkXBZMKhAmRjOS	https://poliscommunity.crown-shy.com
41	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736666020894	1736666020905	\N	\N	c5NIx7aQghEBlux30m6k	https://poliscommunity.crown-shy.com
42	2	\N	\N	1736669336073	1736669336073	\N	\N	\N	\N
43	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736673036849	1736673036865	\N	\N	ouXUdPlFkjPsViT9NlTA	https://poliscommunity.crown-shy.com
44	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736678082194	1736678082208	\N	\N	JQNcpKLdekSNS2jjbUAq	https://poliscommunity.crown-shy.com
45	2	\N	\N	1736679613984	1736679613984	\N	\N	\N	\N
46	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736690333393	1736690333412	\N	\N	uvvLYZ3CMuzpgxIkdFHA	https://poliscommunity.crown-shy.com
47	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736696208668	1736696208687	\N	\N	CIcEdZMVfGjm8cKiXO28	https://poliscommunity.crown-shy.com
48	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736699354398	1736699354413	\N	\N	m8EoaZAJVUI5Q7JCAQzW	https://poliscommunity.crown-shy.com
49	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736699599417	1736699599435	\N	\N	sklqlq8JlS7BjRFlEXaO	https://poliscommunity.crown-shy.com
50	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736699755039	1736699755056	\N	\N	CBugA6hWY0lcYYOQYfIo	https://poliscommunity.crown-shy.com
51	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736699797656	1736699797665	\N	\N	0dH0Ni4kVJA00BDmuage	https://poliscommunity.crown-shy.com
52	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736699984870	1736699984880	\N	\N	SY95Qc0bABKvvnYCE23H	https://poliscommunity.crown-shy.com
53	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736700557257	1736700557274	\N	\N	oY7BLDwLdav8QxfVugZy	https://poliscommunity.crown-shy.com
9	2	\N	\N	1736652321541	1736652321541	\N	t	\N	\N
54	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736700869729	1736700869740	\N	\N	gCmKL92Fp7NJFUJIqBFI	https://poliscommunity.crown-shy.com
55	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736701402278	1736701402292	\N	\N	Z1GxizWsAq9xQUk1CGOR	https://poliscommunity.crown-shy.com
56	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736701484313	1736701484326	\N	\N	9h7xFoDM2ryDQpuzeHLC	https://poliscommunity.crown-shy.com
57	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736701502197	1736701502205	\N	\N	vhmXy5al4hNivxwhRFIS	https://poliscommunity.crown-shy.com
58	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736701670336	1736701670349	\N	\N	JDD9UTTbKMueEGGYfTYU	https://poliscommunity.crown-shy.com
59	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736701789098	1736701789108	\N	\N	NdzT9aq27jhFwJumNwCS	https://poliscommunity.crown-shy.com
60	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736702009813	1736702009825	\N	\N	DchQTMG4oqKDGaSvDqr4	https://poliscommunity.crown-shy.com
61	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736702311361	1736702311375	\N	\N	aiD9mu9Z3VEnqcohNIUI	https://poliscommunity.crown-shy.com
62	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736703188170	1736703188181	\N	\N	s3pMJSFhGKQ1fcE0KoEC	https://poliscommunity.crown-shy.com
63	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736703594984	1736703595001	\N	\N	xfMm0wXwGtISgwUpRVi4	https://poliscommunity.crown-shy.com
64	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736708201398	1736708201415	\N	\N	jUEw4xAzSWwodq2yVWAF	https://poliscommunity.crown-shy.com
65	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736708752146	1736708752159	\N	\N	gzseubX17ouwzD6WjYAq	https://poliscommunity.crown-shy.com
66	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736717867261	1736717867281	\N	\N	IZEOB1w1hdbNt0c8eXLP	https://poliscommunity.crown-shy.com
67	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736719950174	1736719950183	\N	\N	sk4IXQVrSbxgUmyAE8bM	https://poliscommunity.crown-shy.com
68	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736732431757	1736732431774	\N	\N	sB2tw89czuTGLxSxMHyS	https://poliscommunity.crown-shy.com
69	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736734107418	1736734107431	\N	\N	07iP8YLL8JnmDGHOWvgH	https://poliscommunity.crown-shy.com
70	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736734137891	1736734137900	\N	\N	J858719rNh9noO27Ho4l	https://poliscommunity.crown-shy.com
71	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736734372925	1736734372936	\N	\N	21yYfo4h0zg83DhGMggT	https://poliscommunity.crown-shy.com
72	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736734393743	1736734393754	\N	\N	aoR06jvo9KGlzVOOqUFV	https://poliscommunity.crown-shy.com
73	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736734566272	1736734566290	\N	\N	Ean0J0N24ZTJTOw8soxQ	https://poliscommunity.crown-shy.com
74	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736735101030	1736735101043	\N	\N	ueu99LOKoqfon1zTqAol	https://poliscommunity.crown-shy.com
75	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736735626108	1736735626123	\N	\N	btsGlbsOnXGRueDSCUe4	https://poliscommunity.crown-shy.com
76	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736735670826	1736735670832	\N	\N	UsQArUG5SGkMedZqujwT	https://poliscommunity.crown-shy.com
77	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736735704381	1736735704391	\N	\N	p8rYDthB1clntWRQFujw	https://poliscommunity.crown-shy.com
78	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736736427810	1736736427825	\N	\N	1gqrrfGOAflMAB0j1Ecd	https://poliscommunity.crown-shy.com
79	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736736463989	1736736464000	\N	\N	UEkhxJIKXpFkB42dkeVn	https://poliscommunity.crown-shy.com
80	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736736532716	1736736532728	\N	\N	pkv1Bd2Ai9k3xr4vQp89	https://poliscommunity.crown-shy.com
81	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736736989014	1736736989028	\N	\N	1CcUEszkiwVPH4k4DhCt	https://poliscommunity.crown-shy.com
82	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736738953566	1736738953579	\N	\N	1gT60OYRqQjrBDXZd8li	https://poliscommunity.crown-shy.com
83	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736740161595	1736740161608	\N	\N	xgAFm4eymckp8WoygqM6	https://poliscommunity.crown-shy.com
84	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736742667020	1736742667036	\N	\N	c0nrx2DECMY8FvkiWyOn	https://poliscommunity.crown-shy.com
85	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736744241294	1736744241311	\N	\N	JkNhoqIGgzDRtYAxGFuk	https://poliscommunity.crown-shy.com
2	4	\N	\N	1736754940863	1736754940863	\N	\N	\N	\N
86	4	https://poliscommunity.crown-shy.com/5d7tssersj	\N	1736756067755	1736756067765	\N	\N	FtiZnT7kGSCgih70N9V4	https://poliscommunity.crown-shy.com
2	5	\N	\N	1736789772068	1736789772068	\N	\N	\N	\N
87	5	https://poliscommunity.crown-shy.com/4hnrxyx4kr	\N	1736794349411	1736794349425	\N	\N	KlgvSXFT80ryipcLLYst	https://poliscommunity.crown-shy.com
47	5	https://poliscommunity.crown-shy.com/4hnrxyx4kr	\N	1736794422812	1736794422824	\N	\N	CIcEdZMVfGjm8cKiXO28	https://poliscommunity.crown-shy.com
88	5	https://poliscommunity.crown-shy.com/4hnrxyx4kr	\N	1736794459149	1736794459162	\N	\N	eJHTN6rwdSBbwxeGomJM	https://poliscommunity.crown-shy.com
89	5	\N	\N	1736794490274	1736794490274	\N	\N	\N	\N
90	5	https://poliscommunity.crown-shy.com/4hnrxyx4kr	\N	1736794871124	1736794871139	\N	\N	6jZ2bGLu7XVCzqYpmbnf	https://poliscommunity.crown-shy.com
91	5	https://poliscommunity.crown-shy.com/4hnrxyx4kr	\N	1736795450841	1736795450857	\N	\N	Lw2wtgNCFmDA5CwABwJH	https://poliscommunity.crown-shy.com
92	5	https://poliscommunity.crown-shy.com/4hnrxyx4kr	\N	1736795556238	1736795556250	\N	\N	VmlmRwqps0IhCfyqagHH	https://poliscommunity.crown-shy.com
93	5	https://poliscommunity.crown-shy.com/4hnrxyx4kr	\N	1736795556738	1736795556749	\N	\N	xIaxH3odf0qH7vw0sOUM	https://poliscommunity.crown-shy.com
94	5	https://poliscommunity.crown-shy.com/4hnrxyx4kr	\N	1736795587110	1736795587121	\N	\N	nYyvh2NtM2N6lkDH9Dk3	https://poliscommunity.crown-shy.com
23	5	https://poliscommunity.crown-shy.com/4hnrxyx4kr	\N	1736797077182	1736797077203	\N	\N	4o0GqdNNBToizmzCeqTH	https://poliscommunity.crown-shy.com
95	5	https://poliscommunity.crown-shy.com/4hnrxyx4kr	\N	1736797828985	1736797828996	\N	\N	aptfKLDhd78QRVRV1DQK	https://poliscommunity.crown-shy.com
96	5	https://poliscommunity.crown-shy.com/4hnrxyx4kr	\N	1736801499008	1736801499022	\N	\N	ai8VoFkd155zZvfirRH9	https://poliscommunity.crown-shy.com
97	5	https://poliscommunity.crown-shy.com/4hnrxyx4kr	\N	1736802790665	1736802790687	\N	\N	pAWsP1mXutJ7OMc7BN79	https://poliscommunity.crown-shy.com
98	5	https://poliscommunity.crown-shy.com/4hnrxyx4kr	\N	1736803832845	1736803832859	\N	\N	SSWm9JhCCPcyGV90r7nD	https://poliscommunity.crown-shy.com
99	5	https://poliscommunity.crown-shy.com/4hnrxyx4kr	\N	1736804657912	1736804657928	\N	\N	U0gzwgiPkUq73OVL6TmF	https://poliscommunity.crown-shy.com
42	5	\N	\N	1736807068208	1736807068208	\N	\N	\N	\N
100	5	https://poliscommunity.crown-shy.com/4hnrxyx4kr	\N	1736809741530	1736809741545	\N	\N	mndJNXECQc398mDKtUSg	https://poliscommunity.crown-shy.com
101	5	https://poliscommunity.crown-shy.com/4hnrxyx4kr	\N	1736814068068	1736814068079	\N	\N	EWYKgzKyylRMKoBuVDfW	https://poliscommunity.crown-shy.com
102	5	https://poliscommunity.crown-shy.com/4hnrxyx4kr	\N	1736814214352	1736814214360	\N	\N	cOPZGzyZV5XAIDs9t01M	https://poliscommunity.crown-shy.com
103	5	https://poliscommunity.crown-shy.com/4hnrxyx4kr	\N	1736814454244	1736814454256	\N	\N	yjGMiSzqER1Hvk3L2HRz	https://poliscommunity.crown-shy.com
104	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736826738919	1736826738933	\N	\N	kdsJgaYxoL4joFA7knp2	https://poliscommunity.crown-shy.com
105	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736879781072	1736879781087	\N	\N	sFrgNXZtID0vnHNIVW40	https://poliscommunity.crown-shy.com
106	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736879802974	1736879802987	\N	\N	aZXCMGkMbWCgXCbMudg2	https://poliscommunity.crown-shy.com
107	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736879876168	1736879876177	\N	\N	e5bbuBBZtBvl8RQIBKyK	https://poliscommunity.crown-shy.com
108	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736880375719	1736880375738	\N	\N	pBLDPgVlzss86I8TpWR6	https://poliscommunity.crown-shy.com
109	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736880472155	1736880472166	\N	\N	kMf2vvIfbO14VzWvUer1	https://poliscommunity.crown-shy.com
110	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736880731776	1736880731788	\N	\N	y7twPpOWO9TNnQekGuLO	https://poliscommunity.crown-shy.com
111	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736881628271	1736881628288	\N	\N	u7zHxo9h8sDQwSPc4VNE	https://poliscommunity.crown-shy.com
112	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736882248295	1736882248308	\N	\N	F08CJJdEIb7tvGq9SrKD	https://poliscommunity.crown-shy.com
113	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736916054109	1736916054126	\N	\N	nF1Z3aEwoBbSdKCjncRN	https://poliscommunity.crown-shy.com
114	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1736993941548	1736993941568	\N	\N	kiGbYzmcIdaR04F0esx6	https://poliscommunity.crown-shy.com
115	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1737002015146	1737002015154	\N	\N	acmirtLjon7dTj6nEHOF	https://poliscommunity.crown-shy.com
116	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1737041230340	1737041230358	\N	\N	p5e5H2OQb27gpd5GxxDx	https://poliscommunity.crown-shy.com
114	6	\N	\N	1737084575538	1737084575538	kevinlucas1987@gmail.com	\N	\N	\N
117	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1737265238143	1737265238156	\N	\N	LLdFotXMvrbG924BQfzb	https://poliscommunity.crown-shy.com
118	2	https://poliscommunity.crown-shy.com/9huj328rfh	\N	1737467377042	1737467377056	\N	\N	sasberIBluthyuNYQrCs	https://poliscommunity.crown-shy.com
119	3	\N	\N	1737730363470	1737730363470	\N	\N	\N	\N
122	11	\N	\N	1739296894595	1739296894595	\N	\N	\N	\N
124	11	https://poliscommunity.crown-shy.com/6hf7mhtmp3	\N	1739296921430	1739296921437	\N	\N	EaUaTWaBtiZewWzVt9fc	https://poliscommunity.crown-shy.com
125	11	https://poliscommunity.crown-shy.com/6hf7mhtmp3	\N	1739296933276	1739296933288	\N	\N	nnqbJ6ZOkbddKZLvgjW6	https://poliscommunity.crown-shy.com
126	11	https://poliscommunity.crown-shy.com/6hf7mhtmp3	\N	1739296942903	1739296942913	\N	\N	qAj1vRia4J8supOpX6fz	https://poliscommunity.crown-shy.com
127	11	https://poliscommunity.crown-shy.com/6hf7mhtmp3	\N	1739296950353	1739296950360	\N	\N	rwFsUSmt6Jw0yVU5r0BX	https://poliscommunity.crown-shy.com
128	11	https://poliscommunity.crown-shy.com/6hf7mhtmp3	\N	1739296959215	1739296959221	\N	\N	MRHHhnsxFtKbJ69JrwLn	https://poliscommunity.crown-shy.com
129	11	https://poliscommunity.crown-shy.com/6hf7mhtmp3	\N	1739296969745	1739296969752	\N	\N	bJSmkr1xyy4ibLOxvbcU	https://poliscommunity.crown-shy.com
130	11	https://poliscommunity.crown-shy.com/6hf7mhtmp3	\N	1739296989789	1739296989804	\N	\N	TLRaSojqTzfmB7R7QPpk	https://poliscommunity.crown-shy.com
131	11	https://poliscommunity.crown-shy.com/6hf7mhtmp3	\N	1739296989969	1739296989989	\N	\N	TLRaSojqTzfmB7R7QPpk	https://poliscommunity.crown-shy.com
119	11	\N	\N	1739302658045	1739302658045	\N	\N	\N	\N
132	11	https://poliscommunity.crown-shy.com/6hf7mhtmp3?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser3.jpeg&xid=undefined&x_name=Participant	\N	1739308139078	1739308139100	\N	\N	jcyA87OzceDCgOqYnhev	https://poliscommunity.crown-shy.com
133	11	https://poliscommunity.crown-shy.com/6hf7mhtmp3?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser3.jpeg&xid=0ba133fb-3c21-44cd-8a43-8ed0ab6ddda5&x_name=Participant	\N	1739308727046	1739308727065	\N	\N	jcyA87OzceDCgOqYnhev	https://poliscommunity.crown-shy.com
135	11	https://poliscommunity.crown-shy.com/6hf7mhtmp3?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser3.jpeg&x_name=Anonymous&xid=6c4ac217-648e-4206-96d3-bb8789cba4e4	\N	1739316195960	1739316195974	\N	\N	BnXhTTmFOPSFNy6FLxWg	https://poliscommunity.crown-shy.com
170	9	\N	\N	1739317205036	1739317205036	\N	\N	\N	\N
137	9	\N	\N	1739317231700	1739317231700	\N	\N	\N	\N
171	11	\N	\N	1739318781054	1739318781054	\N	\N	\N	\N
172	9	https://poliscommunity.crown-shy.com/8p5dmt5dut	\N	1739345825839	1739345825855	\N	\N	uxUMdjZd02UR7hocbfwV	https://poliscommunity.crown-shy.com
173	11	https://poliscommunity.crown-shy.com/6hf7mhtmp3?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser2.jpeg&x_name=Anonymous&xid=4928b2f6-15ea-4623-94d5-6680b913bc45	\N	1739362870260	1739362870273	\N	\N	opYWNv1FA7LjYYNfVe4J	https://poliscommunity.crown-shy.com
174	11	https://poliscommunity.crown-shy.com/6hf7mhtmp3?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser7.jpeg&x_name=Anonymous&xid=f69ed4f0-db5f-47bc-bfad-b3f77c205ad0	\N	1739363141562	1739363141575	\N	\N	HUP8zG8M6cO9kXVjnFJX	https://poliscommunity.crown-shy.com
175	9	https://poliscommunity.crown-shy.com/8p5dmt5dut	\N	1739371790526	1739371790536	\N	\N	S3KpshgPd6s8J6WczwBx	https://poliscommunity.crown-shy.com
176	9	\N	\N	1739378620295	1739378620295	\N	\N	\N	\N
122	9	https://poliscommunity.crown-shy.com/8p5dmt5dut	\N	1739378649922	1739378649934	\N	\N	QBwG6p7pW1sVdgYJnmLc	https://poliscommunity.crown-shy.com
177	9	\N	\N	1739381298436	1739381298436	\N	\N	\N	\N
122	12	\N	\N	1739381900407	1739381900407	\N	\N	\N	\N
137	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser3.jpeg&x_name=Anonymous&xid=ce4f2e72-97d9-4303-8fc5-874b7f6a9e6b	\N	1739382153144	1739382153167	\N	\N	HwRjIxgaoHyxsUytHdNe	https://poliscommunity.crown-shy.com
178	9	https://poliscommunity.crown-shy.com/8p5dmt5dut	\N	1739384417614	1739384417628	\N	\N	es876P7pYYpK7T39q1MT	https://poliscommunity.crown-shy.com
179	9	\N	\N	1739453811229	1739453811229	\N	\N	\N	\N
180	9	https://poliscommunity.crown-shy.com/8p5dmt5dut	\N	1739453994085	1739453994099	\N	\N	D0RDpVAV3G73KT7vWFB0	https://poliscommunity.crown-shy.com
181	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser2.jpeg&x_name=Anonymous&xid=0bcfcea6-e612-46fb-ba4b-2ffbc00235e9	\N	1739454339156	1739454339169	\N	\N	gnMTmshUE9pcG1ktYTPx	https://poliscommunity.crown-shy.com
182	9	https://poliscommunity.crown-shy.com/8p5dmt5dut	\N	1739454374645	1739454374657	\N	\N	asuxDDmrRoicdWy6efaR	https://poliscommunity.crown-shy.com
183	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser4.jpeg&x_name=Anonymous&xid=030ae34e-8658-4517-99db-0c297ca651ab	\N	1739454392198	1739454392208	\N	\N	s97H6YiRT7EKHAop8pL1	https://poliscommunity.crown-shy.com
184	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser4.jpeg&x_name=Anonymous&xid=b939cdb9-faa9-49a5-b1ac-726aeb6174c0	\N	1739454485142	1739454485154	\N	\N	EYH59341UBU8hupOyTRh	https://poliscommunity.crown-shy.com
185	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser7.jpeg&x_name=Anonymous&xid=452d2f4e-7434-4bc0-bb67-fa47b96d9e6e	\N	1739454492648	1739454492656	\N	\N	PZdUKsAJdNxr2W9tjYOG	https://poliscommunity.crown-shy.com
186	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser4.jpeg&x_name=Anonymous&xid=35c6ca06-2e7a-41ba-8f33-41ce79fa09e6	\N	1739454565972	1739454565985	\N	\N	0dR4YYzwZCnZQPMj7kEI	https://poliscommunity.crown-shy.com
187	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser2.jpeg&x_name=Anonymous&xid=9c4f85ad-8a2c-41fc-aaed-3548a5c431e8	\N	1739454612379	1739454612390	\N	\N	MbCTxQ5f2tiOmFTsjL4y	https://poliscommunity.crown-shy.com
188	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser7.jpeg&x_name=Anonymous&xid=7eaa9350-11a7-4aac-85b8-8e2c1b509e8a	\N	1739454612436	1739454612441	\N	\N	xeKWYka6dtIH2hcemb9N	https://poliscommunity.crown-shy.com
189	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser4.jpeg&x_name=Anonymous&xid=2c7dfd50-cfb4-4f9c-8b21-ad78a1eb0ff7	\N	1739454624477	1739454624487	\N	\N	PrXOZ4s8JfPnVLmfzzDN	https://poliscommunity.crown-shy.com
190	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser5.jpeg&x_name=Anonymous&xid=a1e65b25-337d-4d2b-af3e-cc2257da4a1a	\N	1739454654363	1739454654378	\N	\N	I8R82TiqNKYSsxkpcXzo	https://poliscommunity.crown-shy.com
191	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser1.jpeg&x_name=Anonymous&xid=92908b97-81d7-4c0b-8f6e-67de1993e429	\N	1739454656447	1739454656456	\N	\N	5l8eFJIxTTjVregNyb4K	https://poliscommunity.crown-shy.com
192	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser2.jpeg&x_name=Anonymous&xid=81e56552-f431-44c6-a23c-8aa7c0d051dc	\N	1739454663338	1739454663347	\N	\N	D0RDpVAV3G73KT7vWFB0	https://poliscommunity.crown-shy.com
193	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser6.jpeg&x_name=Anonymous&xid=5e3f6547-e8ec-46bf-8d38-14d80e2a6e54	\N	1739454664834	1739454664840	\N	\N	aoWXiwk4VvZQiLcMmcjS	https://poliscommunity.crown-shy.com
194	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser1.jpeg&x_name=Anonymous&xid=aa1232eb-f93a-4dcf-b2a7-933a3db65f18	\N	1739454686948	1739454686958	\N	\N	ADxtHxzSlkdydagG58MB	https://poliscommunity.crown-shy.com
195	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser1.jpeg&x_name=Anonymous&xid=6efafc2f-e31b-438f-b520-d95f79642f66	\N	1739454690563	1739454690575	\N	\N	PdvoVS4DguQk2G5wcEzW	https://poliscommunity.crown-shy.com
196	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser7.jpeg&x_name=Anonymous&xid=f8f1c1bd-b47f-43d4-9629-d2c95ff01cd7	\N	1739454692399	1739454692405	\N	\N	6qvgDjupy5rZOuJjGcrx	https://poliscommunity.crown-shy.com
197	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser2.jpeg&x_name=Anonymous&xid=3137f512-59d9-4627-881a-2903e12f8758	\N	1739454699293	1739454699300	\N	\N	z2CLKqBzRdnBtzO445tU	https://poliscommunity.crown-shy.com
198	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser3.jpeg&x_name=Anonymous&xid=1da61af9-df92-4377-858e-e85d623648d0	\N	1739454704510	1739454704518	\N	\N	q71vlHobaPa8tWizEs3p	https://poliscommunity.crown-shy.com
199	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser7.jpeg&x_name=Anonymous&xid=8e95cbc1-00fc-472b-aeb0-10d75351e80b	\N	1739454706645	1739454706651	\N	\N	1cfYKFFZz36Jhadlt4Fy	https://poliscommunity.crown-shy.com
200	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser6.jpeg&x_name=Anonymous&xid=5f1c4dc6-f1d6-45a1-8e75-91741997da28	\N	1739454718135	1739454718149	\N	\N	sHulE8ODdBSZWPuw5PsW	https://poliscommunity.crown-shy.com
201	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser4.jpeg&x_name=Anonymous&xid=0d75710c-9671-40db-ba74-ad979a484cce	\N	1739454720102	1739454720109	\N	\N	U1U7tjJvd6BYqx3siutT	https://poliscommunity.crown-shy.com
202	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser2.jpeg&x_name=Anonymous&xid=79921237-3899-4877-99d2-62fc33bb4bbc	\N	1739454726512	1739454726517	\N	\N	asuxDDmrRoicdWy6efaR	https://poliscommunity.crown-shy.com
203	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser5.jpeg&x_name=Anonymous&xid=e0109ba5-056f-4af6-bad6-54ccff95d415	\N	1739454734187	1739454734192	\N	\N	EYRYNku8zy4Qflfos6LF	https://poliscommunity.crown-shy.com
204	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser6.jpeg&x_name=Anonymous&xid=7b63c9c5-fe9b-44d6-bdb7-8d0c567f5342	\N	1739454734751	1739454734755	\N	\N	blEYxNjzUXRqkNacEsqX	https://poliscommunity.crown-shy.com
205	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser5.jpeg&x_name=Anonymous&xid=4e5116de-aaf7-48aa-9b1f-adf0e0c06e2c	\N	1739454774971	1739454774977	\N	\N	Pq6i8ZS2HL5TwB3Ss0iM	https://poliscommunity.crown-shy.com
206	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser3.jpeg&x_name=Anonymous&xid=7c0295e6-4c06-4904-b7d5-571fdebea7d4	\N	1739454852977	1739454852989	\N	\N	rhXQJAJn5VVM8IdOW9lQ	https://poliscommunity.crown-shy.com
207	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser4.jpeg&x_name=Anonymous&xid=47cd06ea-ae75-424b-bc27-ce3410f430f4	\N	1739454891746	1739454891757	\N	\N	zPyA1aRG56Oa9Ab22tWp	https://poliscommunity.crown-shy.com
208	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser4.jpeg&x_name=Anonymous&xid=34091f40-0822-4246-afa3-144e1adc0091	\N	1739454999010	1739454999021	\N	\N	pKFjd00TQ2Tkj1yy8gN6	https://poliscommunity.crown-shy.com
209	9	https://poliscommunity.crown-shy.com/8p5dmt5dut	\N	1739455389387	1739455389397	\N	\N	NmEXUTFizGmQZp5VvILc	https://poliscommunity.crown-shy.com
210	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser1.jpeg&x_name=Anonymous&xid=55e229f3-4b57-4930-a7ed-844910104ab3	\N	1739455585251	1739455585261	\N	\N	woez8TczgTdXcPEFbxvu	https://poliscommunity.crown-shy.com
211	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser1.jpeg&x_name=Anonymous&xid=4b7326b8-6d23-48b2-b416-df2a274caeb8	\N	1739455626197	1739455626204	\N	\N	CQ0rg2Hu65JeAA7qSOO9	https://poliscommunity.crown-shy.com
212	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser4.jpeg&x_name=Anonymous&xid=cf56906c-9eeb-447a-9b32-504fcbaed5c4	\N	1739456152926	1739456152939	\N	\N	ryoY5Or22pw7YTs4AWWx	https://poliscommunity.crown-shy.com
213	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser7.jpeg&x_name=Anonymous&xid=cd33f318-6c78-41d9-a9ee-b59a19f53c8d	\N	1739456444961	1739456444968	\N	\N	OP4uP6Fd0EU8drxFQh0F	https://poliscommunity.crown-shy.com
214	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser3.jpeg&x_name=Anonymous&xid=74887b4d-b07b-4cb4-bd64-5801f0d30abf	\N	1739457739220	1739457739233	\N	\N	h3IA6eIaPqxBeu7Dw6p1	https://poliscommunity.crown-shy.com
215	9	https://poliscommunity.crown-shy.com/8p5dmt5dut	\N	1739457806413	1739457806428	\N	\N	hKTKRnLqHFM4TflUi8nd	https://poliscommunity.crown-shy.com
216	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser6.jpeg&x_name=Anonymous&xid=b7d387c3-15e3-4416-b3e5-c137ccfcc4fe	\N	1739458134524	1739458134533	\N	\N	u23gouN7ccRgFU7kTnnV	https://poliscommunity.crown-shy.com
217	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser5.jpeg&x_name=Anonymous&xid=5dafe606-d24a-42fc-b1ae-0075993b8ce4	\N	1739459642295	1739459642305	\N	\N	A3QRnriRxeMIEGQbLCDh	https://poliscommunity.crown-shy.com
218	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser1.jpeg&x_name=Anonymous&xid=13944e6a-e2d1-4eec-873e-2fdc643b67f1	\N	1739459655463	1739459655476	\N	\N	6bSb2eInxsZNdsy0Fpjd	https://poliscommunity.crown-shy.com
219	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser7.jpeg&x_name=Anonymous&xid=ebf516d7-d20b-4fef-a36b-442b049971b3	\N	1739459703344	1739459703365	\N	\N	edfiR1qZeRbm9uqvkQZh	https://poliscommunity.crown-shy.com
220	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser5.jpeg&x_name=Anonymous&xid=fefd4257-0de6-4ef0-9695-560ec0757fca	\N	1739459790992	1739459791005	\N	\N	l5kZykznS9wIl0Waq7p5	https://poliscommunity.crown-shy.com
221	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser4.jpeg&x_name=Anonymous&xid=350313a1-fa8b-478d-b682-f979e2a6b8de	\N	1739459796790	1739459796809	\N	\N	E4RK21cjWlB5yaC6TONa	https://poliscommunity.crown-shy.com
222	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser3.jpeg&x_name=Anonymous&xid=1bb559c5-0465-4ec0-8ef5-49908042d117	\N	1739459805357	1739459805367	\N	\N	krWipBoo590cH3sfr6ed	https://poliscommunity.crown-shy.com
223	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser2.jpeg&x_name=Anonymous&xid=1c1ff7c6-71cf-4fc9-a333-60f8e4fc482c	\N	1739459827582	1739459827592	\N	\N	HFqMQzPaQU3ZpD8Nky8q	https://poliscommunity.crown-shy.com
224	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser2.jpeg&x_name=Anonymous&xid=a29c7263-ddd3-433b-a1fb-92af1aac76c2	\N	1739459887798	1739459887807	\N	\N	I1B7JiPpEIjENc05VX5k	https://poliscommunity.crown-shy.com
225	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser3.jpeg&x_name=Anonymous&xid=9cc3a0ef-8a50-4a8a-a0d3-7b32dbc9deeb	\N	1739459904669	1739459904678	\N	\N	LiuUfQ7xZbUnmXp1Jd5L	https://poliscommunity.crown-shy.com
226	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser7.jpeg&x_name=Anonymous&xid=b7397eb6-5247-4bce-9058-f92443ddcf71	\N	1739460044136	1739460044146	\N	\N	McMgvecHiwT13S2Xj9gg	https://poliscommunity.crown-shy.com
227	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser3.jpeg&x_name=Anonymous&xid=381822dd-4139-4c1f-8dea-7ecba318c59b	\N	1739460441675	1739460441691	\N	\N	GZoHN89FK85QUApjLzKY	https://poliscommunity.crown-shy.com
228	9	https://poliscommunity.crown-shy.com/8p5dmt5dut	\N	1739460658375	1739460658388	\N	\N	yvGujSfGUSOiXlUYfFQt	https://poliscommunity.crown-shy.com
229	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser7.jpeg&x_name=Anonymous&xid=0ef851b4-9d53-4694-b4bc-5f8b22f73cb2	\N	1739461093140	1739461093156	\N	\N	16ClSJJZ2PASjs9l5HBr	https://poliscommunity.crown-shy.com
230	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser4.jpeg&x_name=Anonymous&xid=64c735c5-eaeb-4d8d-83b2-b6e7723e5425	\N	1739461833739	1739461833751	\N	\N	yvGujSfGUSOiXlUYfFQt	https://poliscommunity.crown-shy.com
231	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser1.jpeg&x_name=Anonymous&xid=fe9b739b-8dae-43e7-b04f-2192c6a473be	\N	1739461875385	1739461875394	\N	\N	wKuff15Nin1Y2ugILUJ1	https://poliscommunity.crown-shy.com
232	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser1.jpeg&x_name=Anonymous&xid=98b85458-c3a2-40ab-8281-88b5f47ffeeb	\N	1739461878725	1739461878733	\N	\N	DzvPt1r8HMzzO3z6NDLm	https://poliscommunity.crown-shy.com
233	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser5.jpeg&x_name=Anonymous&xid=55c9736c-e90f-4628-aa50-745de8d77c0d	\N	1739461906748	1739461906754	\N	\N	QnrJAc6dPlB7SeWP6GcP	https://poliscommunity.crown-shy.com
234	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser3.jpeg&x_name=Anonymous&xid=ccfa701a-a49c-4fa8-83d9-1fcdecacfafd	\N	1739462267977	1739462267989	\N	\N	EHDT88ad3pbklHCXC964	https://poliscommunity.crown-shy.com
235	9	https://poliscommunity.crown-shy.com/8p5dmt5dut	\N	1739462747992	1739462748005	\N	\N	O4KJti3q5lPeaj2Oksie	https://poliscommunity.crown-shy.com
236	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser5.jpeg&x_name=Anonymous&xid=0fc13665-3364-4aec-ad7b-a646810b0482	\N	1739462818955	1739462818974	\N	\N	pR6SVfx3pFHfx6EENXmC	https://poliscommunity.crown-shy.com
237	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser7.jpeg&x_name=Anonymous&xid=c2f60f28-6d1e-4729-ad72-f989a8911f18	\N	1739462850004	1739462850014	\N	\N	tBKFfD7RGhqlw0VLcnA5	https://poliscommunity.crown-shy.com
238	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser2.jpeg&x_name=Anonymous&xid=e3223bd4-ae76-455c-8b75-66b848557cb1	\N	1739462896446	1739462896459	\N	\N	nOC85OoK2mLSysmso2kZ	https://poliscommunity.crown-shy.com
239	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser5.jpeg&x_name=Anonymous&xid=86408954-a538-4669-9310-980e3055e6f9	\N	1739463386544	1739463386563	\N	\N	uMBNv2M6u2o2eD2fEwWi	https://poliscommunity.crown-shy.com
240	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser5.jpeg&x_name=Anonymous&xid=dc493105-e662-4d9a-9e55-71f6cef4e416	\N	1739463945494	1739463945513	\N	\N	WTJELr39iFcxBQWpAZZM	https://poliscommunity.crown-shy.com
241	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser6.jpeg&x_name=Anonymous&xid=f11f3162-01ad-4497-b354-0add9f979a0a	\N	1739464168422	1739464168433	\N	\N	q8IWzJZvEGGIskLiWrG1	https://poliscommunity.crown-shy.com
242	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser3.jpeg&x_name=Anonymous&xid=b2d69188-483b-41e4-aa3a-e5381740f48f	\N	1739464196988	1739464197003	\N	\N	ifFqmoXHybhSOFAUpHmD	https://poliscommunity.crown-shy.com
243	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser6.jpeg&x_name=Anonymous&xid=00ce2571-b23b-4975-bdb1-adbfbd882b52	\N	1739465962080	1739465962087	\N	\N	dmTHEtdgPz1eLSSBowDt	https://poliscommunity.crown-shy.com
245	12	https://poliscommunity.crown-shy.com/9pjrcuzmfh?x_profile_image_url=https%3A%2F%2Fcrownshy.s3.eu-west-2.amazonaws.com%2Fparis_ai_icons%2Fuser5.jpeg&x_name=Anonymous&xid=94bcb277-e677-4518-8eb3-9b71752780ec	\N	1739595941187	1739595941200	\N	\N	S0i9Nxts3KxnZlVMcCi4	https://poliscommunity.crown-shy.com
\.


--
-- Data for Name: permanentcookiezidjoins; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.permanentcookiezidjoins (zid, cookie, created) FROM stdin;
\.


--
-- Data for Name: pwreset_tokens; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.pwreset_tokens (uid, created, token) FROM stdin;
\.


--
-- Data for Name: report_comment_selections; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.report_comment_selections (zid, rid, tid, selection, modified) FROM stdin;
\.


--
-- Data for Name: reports; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.reports (rid, report_id, zid, created, modified, report_name, label_x_neg, label_x_pos, label_y_neg, label_y_pos, label_group_0, label_group_1, label_group_2, label_group_3, label_group_4, label_group_5, label_group_6, label_group_7, label_group_8, label_group_9) FROM stdin;
1	r2267dx22wdejndjzwefh	3	1736651852330	1736651852330	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N
2	r3c7faja9rjhks9fh6zcm	2	1736666569969	1736666569969	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N
3	r4yadsd9mhaa67mawnedr	2	1736733235998	1736733235998	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N
4	r4vwxeebrcsrybhbynuyz	5	1736794853641	1736794853641	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N
5	r8murxahppdmfihwtedta	11	1739297000069	1739297000069	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N
6	r5urummchjcjnrkac6h8y	9	1739383095702	1739383095702	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N
7	r9vbp8xhiyvs36die55h4	12	1739455883076	1739455883076	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N
8	r4nruh5zerteufanba8vk	12	1739456138103	1739456138103	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N
9	r7a2kwdccjmanyf6dcjjv	12	1739456668304	1739456668304	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N
10	r8du9p4ecezjs5mnbsrzd	12	1739457380745	1739457380745	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N
11	r75wwvjjpijdhueb7ay3n	12	1739457816854	1739457816854	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N
12	r9fm7r2ckzndzkexd9sph	12	1739458332530	1739458332530	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N
13	r54scmxz5deemehv3hawa	12	1739459332979	1739459332979	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N
14	r9rmc8st9tnueedkxmyfn	12	1739459332981	1739459332981	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N
15	r5kj7m6ea2wsjzdbikxfd	12	1739459501630	1739459501630	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N
16	r4hvbzvyyfchpebaynmdb	12	1739465057108	1739465057108	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N
17	r4jtthcbey7bekcwmcpny	12	1739466740784	1739466740784	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N
\.


--
-- Data for Name: site_domain_whitelist; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.site_domain_whitelist (site_id, domain_whitelist, domain_whitelist_override_key, modified, created) FROM stdin;
\.


--
-- Data for Name: social_settings; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.social_settings (uid, polis_pic) FROM stdin;
\.


--
-- Data for Name: stars; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.stars (zid, pid, tid, starred, created) FROM stdin;
\.


--
-- Data for Name: suzinvites; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.suzinvites (owner, zid, xid, created, suzinvite) FROM stdin;
\.


--
-- Data for Name: trashes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.trashes (zid, pid, tid, trashed, created) FROM stdin;
\.


--
-- Data for Name: twitter_users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.twitter_users (uid, twitter_user_id, screen_name, name, followers_count, friends_count, verified, profile_image_url_https, location, response, modified, created) FROM stdin;
\.


--
-- Data for Name: upvotes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.upvotes (uid, zid) FROM stdin;
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (uid, hname, created, username, email, is_owner, zinvite, oinvite, tut, site_id, site_owner) FROM stdin;
1	stuart	1736635867628	\N	stuart@crown-shy.com	t	\N	\N	0	polis_site_id_xivpZzcIY1d7Q7SyKL	t
2	happy_mountain	1736644681447	\N	shanhuan@proton.me	t	\N	\N	0	polis_site_id_3JV5lURmYDyQDEOHF9	t
3	\N	1736648226786	\N	\N	f	\N	\N	0	polis_site_id_BHnZ6xkQMM7OiXukw3	t
4	\N	1736651755432	\N	\N	f	\N	\N	0	polis_site_id_NSoK6WvPJb4y3Nx6uP	t
5	\N	1736651769837	\N	\N	f	\N	\N	0	polis_site_id_vbdQ89wQ9g7B69fGLP	t
6	\N	1736651789073	\N	\N	f	\N	\N	0	polis_site_id_TCFWnW7R1vJPywMYxp	t
7	\N	1736651807256	\N	\N	f	\N	\N	0	polis_site_id_LTC0IqA3Mvl1W3zzAQ	t
8	\N	1736651833915	\N	\N	f	\N	\N	0	polis_site_id_wCK7fZz8bajEQCMMin	t
9	\N	1736652321499	\N	\N	f	\N	\N	0	polis_site_id_UqNJFfXAuvb4lhZgXz	t
10	\N	1736652375049	\N	\N	f	\N	\N	0	polis_site_id_Kkoc3kiHmK7PIeinUi	t
11	\N	1736653653709	\N	\N	f	\N	\N	0	polis_site_id_hVvRCk194ooWFa9QCY	t
12	\N	1736653654752	\N	\N	f	\N	\N	0	polis_site_id_IC0bRAEHhfeFPlle0j	t
13	\N	1736654381023	\N	\N	f	\N	\N	0	polis_site_id_6zX49cjSgIaEMRLqyb	t
14	\N	1736654517906	\N	\N	f	\N	\N	0	polis_site_id_8vJZfp4y2X4ZeQlPr7	t
15	\N	1736654561413	\N	\N	f	\N	\N	0	polis_site_id_7aYbeXqiFI9K3CU5u7	t
16	\N	1736654654150	\N	\N	f	\N	\N	0	polis_site_id_NrgK1XX0SzYJVUMhPP	t
17	\N	1736654747955	\N	\N	f	\N	\N	0	polis_site_id_TL9GmW4raq2rYL0m4p	t
18	\N	1736654854100	\N	\N	f	\N	\N	0	polis_site_id_YIVaRqTPIKyxh0mica	t
19	\N	1736654927189	\N	\N	f	\N	\N	0	polis_site_id_dqdsPFHlt8XsoEmpnT	t
20	\N	1736655204339	\N	\N	f	\N	\N	0	polis_site_id_kG4LK7aDQT4gcGRT9F	t
21	\N	1736655285235	\N	\N	f	\N	\N	0	polis_site_id_oOlK8q2aBYE0V559wq	t
22	\N	1736655354499	\N	\N	f	\N	\N	0	polis_site_id_1HfX4UUtJ9YKhtKkaG	t
23	\N	1736656290973	\N	\N	f	\N	\N	0	polis_site_id_l3o04Cfa1xN3tRFdpB	t
24	\N	1736656341003	\N	\N	f	\N	\N	0	polis_site_id_dETebCsLOX7QqEmGH5	t
25	\N	1736656787590	\N	\N	f	\N	\N	0	polis_site_id_ULYDkaJ9DqtYZV1dh1	t
26	\N	1736657017413	\N	\N	f	\N	\N	0	polis_site_id_2Gs904yYBRO3N583oh	t
27	\N	1736657028347	\N	\N	f	\N	\N	0	polis_site_id_BXKyF9PybwdvzEexx9	t
28	\N	1736657159868	\N	\N	f	\N	\N	0	polis_site_id_Nu7KGuDSfHzdob9gZw	t
29	\N	1736657170147	\N	\N	f	\N	\N	0	polis_site_id_8YmcnNwn6jp84F95Bl	t
30	\N	1736657185839	\N	\N	f	\N	\N	0	polis_site_id_j0Ukw67HrNpkFbf0dM	t
31	\N	1736657209871	\N	\N	f	\N	\N	0	polis_site_id_iLSYHZowshR1gYhDd4	t
32	\N	1736657211850	\N	\N	f	\N	\N	0	polis_site_id_drzah8JRzzMvYVrTXU	t
33	\N	1736657594904	\N	\N	f	\N	\N	0	polis_site_id_PTojroJewL62oEeBer	t
34	\N	1736657746076	\N	\N	f	\N	\N	0	polis_site_id_Rhj0wHlF0aYvsLiIX3	t
35	\N	1736657793164	\N	\N	f	\N	\N	0	polis_site_id_NkdOH7ruEIxjZr8Tun	t
36	\N	1736657847046	\N	\N	f	\N	\N	0	polis_site_id_xFJ6NUZSjCvFX4gUFv	t
37	\N	1736657926173	\N	\N	f	\N	\N	0	polis_site_id_BgfYdnPLpecFCMItEK	t
38	\N	1736658160240	\N	\N	f	\N	\N	0	polis_site_id_wl8VotYGGFiECI0VOC	t
39	\N	1736663252718	\N	\N	f	\N	\N	0	polis_site_id_ginkzFA87NLZXAcIgY	t
40	\N	1736664442684	\N	\N	f	\N	\N	0	polis_site_id_cLcJDt2ibiC1HzE2Tp	t
41	\N	1736666020878	\N	\N	f	\N	\N	0	polis_site_id_UbaE6tSUuq5DfirkU2	t
42	\N	1736669336039	\N	\N	f	\N	\N	0	polis_site_id_USyMyrtz7SWa3Su2cz	t
43	\N	1736673036828	\N	\N	f	\N	\N	0	polis_site_id_gAl7Hqo17yaVmy6T3P	t
44	\N	1736678082176	\N	\N	f	\N	\N	0	polis_site_id_FYUY0m6Ree9jFq3pmQ	t
45	\N	1736679613952	\N	\N	f	\N	\N	0	polis_site_id_t6iHtjSerToyMMjw24	t
46	\N	1736690333369	\N	\N	f	\N	\N	0	polis_site_id_5GJy16eLCBjpqYLnWB	t
47	\N	1736696208641	\N	\N	f	\N	\N	0	polis_site_id_oJVao0hFETT9Ru0Nlv	t
48	\N	1736699354376	\N	\N	f	\N	\N	0	polis_site_id_ZbL7AXMbjqgVyVRhbu	t
49	\N	1736699599392	\N	\N	f	\N	\N	0	polis_site_id_LNK58sGSdQVCircXLZ	t
50	\N	1736699755017	\N	\N	f	\N	\N	0	polis_site_id_GEelmkAZqSpIIxPa97	t
51	\N	1736699797636	\N	\N	f	\N	\N	0	polis_site_id_UnxgPsJ5tq8VasD4cE	t
52	\N	1736699984853	\N	\N	f	\N	\N	0	polis_site_id_V2ILEt5o4jrHH8XQZn	t
53	\N	1736700557234	\N	\N	f	\N	\N	0	polis_site_id_ALCX8Hoa3mVdyeCjdi	t
54	\N	1736700869716	\N	\N	f	\N	\N	0	polis_site_id_muT8zEp6EVvloyGecr	t
55	\N	1736701402256	\N	\N	f	\N	\N	0	polis_site_id_7FMQU46Jj5e41ojaoU	t
56	\N	1736701484293	\N	\N	f	\N	\N	0	polis_site_id_G5GKUYV3DlJJ3BsquI	t
57	\N	1736701502183	\N	\N	f	\N	\N	0	polis_site_id_wtDzVUjRQvTMj3FZpd	t
58	\N	1736701670318	\N	\N	f	\N	\N	0	polis_site_id_uRF1dtDIwrnZAJNvvb	t
59	\N	1736701789086	\N	\N	f	\N	\N	0	polis_site_id_tJttJFINJYs99cP2D1	t
60	\N	1736702009797	\N	\N	f	\N	\N	0	polis_site_id_RNqHo9elvWhKzMbxwf	t
61	\N	1736702311343	\N	\N	f	\N	\N	0	polis_site_id_w6DyIz35FMr8wqLk12	t
62	\N	1736703188150	\N	\N	f	\N	\N	0	polis_site_id_vEghrH0BOaiFEmb7ml	t
63	\N	1736703594962	\N	\N	f	\N	\N	0	polis_site_id_utrldiImns3Gl6hNZX	t
64	\N	1736708201374	\N	\N	f	\N	\N	0	polis_site_id_FqsnDLH9AODRIQtwVu	t
65	\N	1736708752126	\N	\N	f	\N	\N	0	polis_site_id_bR6a5MCSzeVeOEhfL9	t
66	\N	1736717867236	\N	\N	f	\N	\N	0	polis_site_id_oBQyJjXstRMXSayOe7	t
67	\N	1736719950153	\N	\N	f	\N	\N	0	polis_site_id_HGtlsy2yO1NeJ5MlUo	t
68	\N	1736732431731	\N	\N	f	\N	\N	0	polis_site_id_Hu39bTFzeQVOo1wHd8	t
69	\N	1736734107397	\N	\N	f	\N	\N	0	polis_site_id_I0UYzRfVlF1GO7t6rD	t
70	\N	1736734137879	\N	\N	f	\N	\N	0	polis_site_id_U7dSN72Q0GBzZ53EZQ	t
71	\N	1736734372902	\N	\N	f	\N	\N	0	polis_site_id_6GJqQI15sQJfPoDfxE	t
72	\N	1736734393730	\N	\N	f	\N	\N	0	polis_site_id_aKTe0QYgz7RvnjCY9s	t
73	\N	1736734566248	\N	\N	f	\N	\N	0	polis_site_id_kZIFXsh3ElZ40oH1iH	t
74	\N	1736735101009	\N	\N	f	\N	\N	0	polis_site_id_RGwaX95HQPk4F0oJiq	t
75	\N	1736735626091	\N	\N	f	\N	\N	0	polis_site_id_r9d3Z5T5iznPm3cck7	t
76	\N	1736735670818	\N	\N	f	\N	\N	0	polis_site_id_2OoKekYnwvdAzm1bZx	t
77	\N	1736735704364	\N	\N	f	\N	\N	0	polis_site_id_xFw4pp9WRjcGpIzuAm	t
78	\N	1736736427790	\N	\N	f	\N	\N	0	polis_site_id_s6FOGPTRpryLQNQKKd	t
79	\N	1736736463971	\N	\N	f	\N	\N	0	polis_site_id_DDYhtxzD3TpCabuX9g	t
80	\N	1736736532697	\N	\N	f	\N	\N	0	polis_site_id_KG0iMXa2Jyk0V2lfXx	t
81	\N	1736736988994	\N	\N	f	\N	\N	0	polis_site_id_5XbHB2IIQ5mqbn0N6e	t
82	\N	1736738953546	\N	\N	f	\N	\N	0	polis_site_id_DSaEzH9ZknLqP3KL7n	t
83	\N	1736740161576	\N	\N	f	\N	\N	0	polis_site_id_VvH0RHUAJLDX0I61dU	t
84	\N	1736742666996	\N	\N	f	\N	\N	0	polis_site_id_B0fzDM5hoipI81PfkE	t
85	\N	1736744241270	\N	\N	f	\N	\N	0	polis_site_id_JJGdkLd7dKjhncwFnq	t
86	\N	1736756067738	\N	\N	f	\N	\N	0	polis_site_id_7U80fD9xn8GdG68J1X	t
87	\N	1736794349377	\N	\N	f	\N	\N	0	polis_site_id_WSwBopLauNRfpt33Vb	t
88	\N	1736794459130	\N	\N	f	\N	\N	0	polis_site_id_m8MvvutB3rwUg31v94	t
89	\N	1736794490245	\N	\N	f	\N	\N	0	polis_site_id_SPEcju8jR5yJXA3xoy	t
90	\N	1736794871104	\N	\N	f	\N	\N	0	polis_site_id_yarHSExWl7GWepfYDu	t
91	\N	1736795450820	\N	\N	f	\N	\N	0	polis_site_id_3h1fM8Q1gAYaWoDT7w	t
92	\N	1736795556222	\N	\N	f	\N	\N	0	polis_site_id_IzGBEh8vuHcrTSlVkC	t
93	\N	1736795556720	\N	\N	f	\N	\N	0	polis_site_id_ZKSHofUg7RuBr6psWV	t
94	\N	1736795587097	\N	\N	f	\N	\N	0	polis_site_id_b2r172YzswW0bSZupY	t
95	\N	1736797828969	\N	\N	f	\N	\N	0	polis_site_id_mn4fBaVfQyAvX0MHCv	t
96	\N	1736801498983	\N	\N	f	\N	\N	0	polis_site_id_46YF50LWB2rSKLliCo	t
97	\N	1736802782308	\N	\N	f	\N	\N	0	polis_site_id_76AtDvzPUlfhsLOXvL	t
98	\N	1736803832826	\N	\N	f	\N	\N	0	polis_site_id_5mJXHhx8N44o9xuVpa	t
99	\N	1736804657889	\N	\N	f	\N	\N	0	polis_site_id_5M3j0q0AIBEbZF2seC	t
100	\N	1736809741508	\N	\N	f	\N	\N	0	polis_site_id_yVha6uZgYt2tqpBPEe	t
101	\N	1736814068051	\N	\N	f	\N	\N	0	polis_site_id_uFthoVy7acTa6p41sb	t
102	\N	1736814214338	\N	\N	f	\N	\N	0	polis_site_id_XQVxjUYRIeyCG3dJsG	t
103	\N	1736814454223	\N	\N	f	\N	\N	0	polis_site_id_gtIq7da7o6j0BnA8bu	t
104	\N	1736826738898	\N	\N	f	\N	\N	0	polis_site_id_jBsZ18Fdo7hqlRQncV	t
105	\N	1736879781051	\N	\N	f	\N	\N	0	polis_site_id_aN3vKBwjNScwd59xP0	t
106	\N	1736879802954	\N	\N	f	\N	\N	0	polis_site_id_KPdzqyfnw15VrIuLhF	t
107	\N	1736879876153	\N	\N	f	\N	\N	0	polis_site_id_pSX2fAnImg8bq4j1Nb	t
108	\N	1736880375700	\N	\N	f	\N	\N	0	polis_site_id_k2tHoQD8BkcHjYBF3H	t
109	\N	1736880472136	\N	\N	f	\N	\N	0	polis_site_id_yUQmw3Z5PPV9bM9AGa	t
110	\N	1736880731755	\N	\N	f	\N	\N	0	polis_site_id_f1R5qhRKa6C9CXk9on	t
111	\N	1736881628230	\N	\N	f	\N	\N	0	polis_site_id_oORet3hODAZCsrRzXd	t
112	\N	1736882248276	\N	\N	f	\N	\N	0	polis_site_id_3gqmDviw1JdPAVYqjF	t
113	\N	1736916054086	\N	\N	f	\N	\N	0	polis_site_id_Fzg8sy5O5vIcZxkqDd	t
114	Kevin L	1736958241097	\N	kevinlucas1987@gmail.com	t	\N	\N	0	polis_site_id_jUGtTlm1jvDE0Vz4k0	t
115	\N	1737002015135	\N	\N	f	\N	\N	0	polis_site_id_WJ65qJHTjOy6xAz2EC	t
116	\N	1737041230316	\N	\N	f	\N	\N	0	polis_site_id_ac9SI7IA4CsjRNJBll	t
117	\N	1737265238120	\N	\N	f	\N	\N	0	polis_site_id_l7qBviLJRPgglTW3hh	t
118	\N	1737467377019	\N	\N	f	\N	\N	0	polis_site_id_iOm415Yu5cCH0BQ6M8	t
119	\N	1737730363444	\N	\N	f	\N	\N	0	polis_site_id_XMpLU1uTy90Wqoe2G6	t
120	Rowan Harris	1737739896598	\N	rowan.harris@deliberate.org.uk	t	\N	\N	0	polis_site_id_fXdSet79c2uCd1yxDl	t
121	Willow Idlewild	1738358696887	\N	willow@bl00cyb.org	t	\N	\N	0	polis_site_id_7Vt7SaUI1TEkAuGBvR	t
122	paris_ai_summit	1739182644047	\N	parisai@crown-shy.com	t	\N	\N	0	polis_site_id_cQ6uuFMUpuPFnEYoC8	t
123	Christophe Gauthier	1739197644837	\N	christophe.gauthier@leplusimportant.org	t	\N	\N	0	polis_site_id_26jiKVMrv9JfZ4XqlB	t
124	\N	1739296921411	\N	\N	f	\N	\N	0	polis_site_id_e4kYMIo1qVap6i79w9	t
125	\N	1739296933255	\N	\N	f	\N	\N	0	polis_site_id_vgIsv6fle76ZQ5eyWA	t
126	\N	1739296942888	\N	\N	f	\N	\N	0	polis_site_id_J2nDOWx6YbWsAQtw1z	t
127	\N	1739296950341	\N	\N	f	\N	\N	0	polis_site_id_EwHdDio9qNI63A2K2a	t
128	\N	1739296959206	\N	\N	f	\N	\N	0	polis_site_id_k6py2weVR2u7hue3lv	t
129	\N	1739296969732	\N	\N	f	\N	\N	0	polis_site_id_pudzhA50qafawItv3u	t
130	\N	1739296989720	\N	\N	f	\N	\N	0	polis_site_id_OprxtquyQ1S9KWlYeK	t
131	\N	1739296989869	\N	\N	f	\N	\N	0	polis_site_id_I56atC9V53Xs4wL4pe	t
132	\N	1739308084569	\N	\N	f	\N	\N	0	polis_site_id_NywdZORXbNYAmyYK8c	t
133	\N	1739308680102	\N	\N	f	\N	\N	0	polis_site_id_s9lWYwLez5IESTas7I	t
134	\N	1739313565106	\N	\N	f	\N	\N	0	polis_site_id_7PF2RvCmKcHap3DLIW	t
135	\N	1739316193572	\N	\N	f	\N	\N	0	polis_site_id_YEjJDSxPd15sPt1bUp	t
136	\N	1739316970569	\N	\N	f	\N	\N	0	polis_site_id_pIUCDhIN3WAFDLwHjS	t
137	\N	1739317069018	\N	\N	f	\N	\N	0	polis_site_id_WkhUINUAWiGobjOFx8	t
170	\N	1739317205003	\N	\N	f	\N	\N	0	polis_site_id_SKrNrAmxTXrRyWi96D	t
171	\N	1739318781032	\N	\N	f	\N	\N	0	polis_site_id_UJRdrvi3mxq9ytDk9o	t
172	\N	1739345825817	\N	\N	f	\N	\N	0	polis_site_id_zrpDsldlsinIBjPAmI	t
173	\N	1739362870243	\N	\N	f	\N	\N	0	polis_site_id_Um1cgxE7WMbVOnGkh3	t
174	\N	1739363141544	\N	\N	f	\N	\N	0	polis_site_id_OgEpebp3yW7cAQKAF3	t
175	\N	1739371790503	\N	\N	f	\N	\N	0	polis_site_id_kMvL6NfmXzwtzAfCME	t
176	\N	1739378620263	\N	\N	f	\N	\N	0	polis_site_id_sW4jh2TsiCEVwcSZNC	t
177	\N	1739381298390	\N	\N	f	\N	\N	0	polis_site_id_t0h9nBr9NLeSUKOTGt	t
178	\N	1739384417592	\N	\N	f	\N	\N	0	polis_site_id_uKf8WxLhXccZewFDD6	t
179	\N	1739453811196	\N	\N	f	\N	\N	0	polis_site_id_1UV0csHJY0oyF1KBih	t
180	\N	1739453994070	\N	\N	f	\N	\N	0	polis_site_id_xmpzUVhRwKul4E9qZs	t
181	\N	1739454339140	\N	\N	f	\N	\N	0	polis_site_id_CFjsau2WPteM1JuTFS	t
182	\N	1739454374625	\N	\N	f	\N	\N	0	polis_site_id_RcC8dpzHPljBjr3Rha	t
183	\N	1739454392188	\N	\N	f	\N	\N	0	polis_site_id_nQw7maBsgfdPgR2nFE	t
184	\N	1739454485130	\N	\N	f	\N	\N	0	polis_site_id_4AqjNGvZFoOqdYjIIr	t
185	\N	1739454492640	\N	\N	f	\N	\N	0	polis_site_id_EZn3ndFkuripb5rYG8	t
186	\N	1739454565958	\N	\N	f	\N	\N	0	polis_site_id_nsoyFGakZtWa2Fybog	t
187	\N	1739454612368	\N	\N	f	\N	\N	0	polis_site_id_h4ERwfBvVTzpHESLLB	t
188	\N	1739454612428	\N	\N	f	\N	\N	0	polis_site_id_PgnQLF8xjZeODEsSiq	t
189	\N	1739454624466	\N	\N	f	\N	\N	0	polis_site_id_yZYBrFWrURKHcMoUGX	t
190	\N	1739454654346	\N	\N	f	\N	\N	0	polis_site_id_Qzlp77QQMX2NndDDtg	t
191	\N	1739454656437	\N	\N	f	\N	\N	0	polis_site_id_MsfRKCPnC1puw3wrtM	t
192	\N	1739454663328	\N	\N	f	\N	\N	0	polis_site_id_uMEzF18HC5T77JN0m1	t
193	\N	1739454664828	\N	\N	f	\N	\N	0	polis_site_id_MniIsavvWAyQiEjbR4	t
194	\N	1739454686932	\N	\N	f	\N	\N	0	polis_site_id_vO1UtVz7pm0ZalFqbi	t
195	\N	1739454690554	\N	\N	f	\N	\N	0	polis_site_id_CJAvSClJdcne3JWMkk	t
196	\N	1739454692393	\N	\N	f	\N	\N	0	polis_site_id_X9p8m0REX0hszllEwM	t
197	\N	1739454699286	\N	\N	f	\N	\N	0	polis_site_id_Rl9E9vREFCo5B5nmhs	t
198	\N	1739454704501	\N	\N	f	\N	\N	0	polis_site_id_Jgp451wReqkd2I1jLR	t
199	\N	1739454706639	\N	\N	f	\N	\N	0	polis_site_id_bg1ANsqvVgQUUCj5vj	t
200	\N	1739454718121	\N	\N	f	\N	\N	0	polis_site_id_LPsfZrmjYuPUrY8W2X	t
201	\N	1739454720096	\N	\N	f	\N	\N	0	polis_site_id_CJRXkCufV39KqJN2zW	t
202	\N	1739454726506	\N	\N	f	\N	\N	0	polis_site_id_xfiJENZPGCk378fOED	t
203	\N	1739454734182	\N	\N	f	\N	\N	0	polis_site_id_sEXxqYg9yAp0XXV2i5	t
204	\N	1739454734747	\N	\N	f	\N	\N	0	polis_site_id_lLoGTGZ00EkfwsNjwz	t
205	\N	1739454774965	\N	\N	f	\N	\N	0	polis_site_id_8ZfgkMVHB6i8ppSe9G	t
206	\N	1739454852964	\N	\N	f	\N	\N	0	polis_site_id_AN1apPof9s0766rsBZ	t
207	\N	1739454891736	\N	\N	f	\N	\N	0	polis_site_id_6MCUjQ4mO9RZooQkNT	t
208	\N	1739454999001	\N	\N	f	\N	\N	0	polis_site_id_EgW7ufmmQf9e736z0S	t
209	\N	1739455389373	\N	\N	f	\N	\N	0	polis_site_id_GoA4cGNYrmdcfPt10g	t
210	\N	1739455585241	\N	\N	f	\N	\N	0	polis_site_id_uOkGwddkbark0Q2snM	t
211	\N	1739455626185	\N	\N	f	\N	\N	0	polis_site_id_E8C1VB5AnxIYNc9Crk	t
212	\N	1739456152911	\N	\N	f	\N	\N	0	polis_site_id_vpCwTuV8YOodPwJGbq	t
213	\N	1739456444953	\N	\N	f	\N	\N	0	polis_site_id_USwQpZrk9L9wS0MnZV	t
214	\N	1739457739205	\N	\N	f	\N	\N	0	polis_site_id_WsToIwILBvE9sx4vHg	t
215	\N	1739457806391	\N	\N	f	\N	\N	0	polis_site_id_cbEKrLvcz7j1TvWm6g	t
216	\N	1739458134511	\N	\N	f	\N	\N	0	polis_site_id_Q34cHZEplOCrX4EJPa	t
217	\N	1739459642281	\N	\N	f	\N	\N	0	polis_site_id_23pNYWfz1Rj02RyQmZ	t
218	\N	1739459655447	\N	\N	f	\N	\N	0	polis_site_id_eUGqk6mbHRZEKlAKug	t
219	\N	1739459703324	\N	\N	f	\N	\N	0	polis_site_id_B1r8ieTVF1STCzeBAL	t
220	\N	1739459790979	\N	\N	f	\N	\N	0	polis_site_id_UqgmxjXGWGGjglKGtd	t
221	\N	1739459796772	\N	\N	f	\N	\N	0	polis_site_id_z95Pcj2WOMC4hw4LnM	t
222	\N	1739459805347	\N	\N	f	\N	\N	0	polis_site_id_xvjNQzzlaCvxrhmLaE	t
223	\N	1739459827566	\N	\N	f	\N	\N	0	polis_site_id_TJ6C9cBiI86Bx62ptw	t
224	\N	1739459887782	\N	\N	f	\N	\N	0	polis_site_id_fQrS7Ls6JBGTFT55bL	t
225	\N	1739459904659	\N	\N	f	\N	\N	0	polis_site_id_U0CD8ivZxJR5j9Gxk9	t
226	\N	1739460044125	\N	\N	f	\N	\N	0	polis_site_id_s5YTXGHpK4fYvUctdB	t
227	\N	1739460441657	\N	\N	f	\N	\N	0	polis_site_id_Vs02E3rhstKzpq1Olf	t
228	\N	1739460658355	\N	\N	f	\N	\N	0	polis_site_id_5WEqe4d3CvuSyoeBXo	t
229	\N	1739461093122	\N	\N	f	\N	\N	0	polis_site_id_zS83OBUzcAnfLPvXFw	t
230	\N	1739461833721	\N	\N	f	\N	\N	0	polis_site_id_mIOigK0ClERJsVl6Kn	t
231	\N	1739461875375	\N	\N	f	\N	\N	0	polis_site_id_R1Uc3G8a2LLjbXbKRj	t
232	\N	1739461878713	\N	\N	f	\N	\N	0	polis_site_id_DHUSilHELaV4XxjAlV	t
233	\N	1739461906741	\N	\N	f	\N	\N	0	polis_site_id_oASn0XbibxeiLTWOTV	t
234	\N	1739462267962	\N	\N	f	\N	\N	0	polis_site_id_hWfkWqtjxTJkLL7mb8	t
235	\N	1739462747963	\N	\N	f	\N	\N	0	polis_site_id_dODmn9xJ0gzmwfMb4T	t
236	\N	1739462818938	\N	\N	f	\N	\N	0	polis_site_id_xdzj0m4EpFR5F38cfk	t
237	\N	1739462849995	\N	\N	f	\N	\N	0	polis_site_id_gefp1BkyySelSZ9uor	t
238	\N	1739462896433	\N	\N	f	\N	\N	0	polis_site_id_b3JO9LSwYCgsqkGNEa	t
239	\N	1739463386527	\N	\N	f	\N	\N	0	polis_site_id_dLeoh2E8v5ktk6ZIQY	t
240	\N	1739463945481	\N	\N	f	\N	\N	0	polis_site_id_RG1jaXjQKdcKP6IhaY	t
241	\N	1739464168409	\N	\N	f	\N	\N	0	polis_site_id_sf7l1fpEUW2Sq10Fg4	t
242	\N	1739464196954	\N	\N	f	\N	\N	0	polis_site_id_ZtHjZ2ZsAJn4xwnjwJ	t
243	\N	1739465962071	\N	\N	f	\N	\N	0	polis_site_id_IQmqsu1S50gRrokIM9	t
244	Andy	1739539452221	\N	andy@crown-shy.com	t	\N	\N	0	polis_site_id_oiMBl4Q4lxyK9MSXtw	t
245	\N	1739595941171	\N	\N	f	\N	\N	0	polis_site_id_kNWX69yWQE6cUyeSxK	t
246	Liz Barry	1739735279339	\N	liz@crown-shy.com	t	\N	\N	0	polis_site_id_ytWB8WegGsBNMq11vs	t
\.


--
-- Data for Name: votes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.votes (zid, pid, tid, vote, weight_x_32767, created, high_priority) FROM stdin;
2	0	0	0	0	1736645112983	f
2	0	1	0	0	1736645326845	f
2	0	2	0	0	1736645419842	f
2	0	3	0	0	1736645624667	f
2	0	4	0	0	1736645674996	f
2	0	5	0	0	1736645696284	f
2	0	6	0	0	1736645712966	f
2	0	7	0	0	1736645758830	f
2	0	8	0	0	1736645773730	f
2	0	9	0	0	1736645801425	f
2	0	10	0	0	1736645807880	f
2	0	11	0	0	1736645836839	f
2	0	12	0	0	1736645874952	f
2	0	13	0	0	1736646064899	f
2	0	14	0	0	1736646075850	f
2	0	15	0	0	1736646664423	f
2	0	16	0	0	1736647227843	f
2	0	17	0	0	1736647444998	f
2	0	18	0	0	1736647466580	f
2	0	19	0	0	1736647514719	f
2	0	20	0	0	1736647550812	f
3	0	0	0	0	1736648104619	f
3	1	1	-1	0	1736648226888	f
3	1	0	-1	0	1736648231383	f
2	0	21	0	0	1736649660268	f
2	0	22	0	0	1736649697444	f
2	0	23	0	0	1736649752807	f
2	0	24	0	0	1736649905765	f
2	0	25	0	0	1736649945157	f
3	2	2	-1	0	1736651755530	f
3	2	1	-1	0	1736651756417	f
3	2	0	1	0	1736651757184	f
3	3	3	-1	0	1736651769936	f
3	3	2	-1	0	1736651772070	f
3	3	0	1	0	1736651773027	f
3	3	1	-1	0	1736651774016	f
3	4	4	-1	0	1736651789172	f
3	4	2	-1	0	1736651790756	f
3	4	1	1	0	1736651791283	f
3	4	0	-1	0	1736651792289	f
3	4	3	1	0	1736651792955	f
3	5	5	-1	0	1736651807350	f
3	5	4	-1	0	1736651808171	f
3	5	1	1	0	1736651808816	f
3	5	3	-1	0	1736651809966	f
3	5	0	-1	0	1736651810459	f
3	5	2	-1	0	1736651811204	f
3	6	6	-1	0	1736651834055	f
3	6	3	1	0	1736651835848	f
3	6	2	-1	0	1736651836945	f
3	6	4	1	0	1736651837489	f
3	6	1	-1	0	1736651838311	f
3	6	0	-1	0	1736651839495	f
3	6	5	1	0	1736651840729	f
2	1	26	-1	0	1736652321611	f
2	2	2	0	0	1736652375081	f
2	2	6	0	0	1736652376035	f
2	2	1	0	0	1736652377019	f
2	2	23	0	0	1736652378011	f
2	3	23	-1	0	1736653653755	f
2	4	2	0	0	1736653654778	f
2	4	6	1	0	1736653658066	f
2	3	2	-1	0	1736653660734	f
2	4	1	1	0	1736653660985	f
2	3	1	1	0	1736653662981	f
2	4	23	-1	0	1736653666308	f
2	3	25	0	0	1736653668593	f
2	4	5	-1	0	1736653668848	f
2	3	6	1	0	1736653670654	f
2	4	13	-1	0	1736653671431	f
2	3	21	-1	0	1736653674691	f
2	4	11	-1	0	1736653675797	f
2	3	13	-1	0	1736653677713	f
2	4	3	-1	0	1736653678665	f
2	3	5	-1	0	1736653680029	f
2	4	15	-1	0	1736653681394	f
2	4	21	-1	0	1736653684360	f
2	3	24	1	0	1736653684643	f
2	4	20	-1	0	1736653687421	f
2	4	17	-1	0	1736653691263	f
2	3	11	0	0	1736653694323	f
2	3	3	-1	0	1736653696932	f
2	4	24	0	0	1736653698076	f
2	4	9	-1	0	1736653701567	f
2	3	20	-1	0	1736653703041	f
2	4	10	-1	0	1736653703597	f
2	3	15	-1	0	1736653706032	f
2	3	17	-1	0	1736653708640	f
2	3	9	-1	0	1736653711573	f
2	4	22	0	0	1736653712439	f
2	3	10	-1	0	1736653713721	f
2	4	12	-1	0	1736653715561	f
2	4	25	-1	0	1736653718721	f
2	3	22	0	0	1736653722082	f
2	3	12	-1	0	1736653726343	f
2	5	24	1	0	1736654381078	f
2	5	20	-1	0	1736654390375	f
2	5	10	-1	0	1736654393159	f
2	5	1	1	0	1736654395817	f
2	5	2	1	0	1736654402406	f
2	5	25	-1	0	1736654408329	f
2	5	17	-1	0	1736654410766	f
2	5	15	-1	0	1736654413648	f
2	5	21	-1	0	1736654417670	f
2	5	11	-1	0	1736654422591	f
2	5	5	-1	0	1736654425958	f
2	5	6	-1	0	1736654428467	f
2	5	12	-1	0	1736654433551	f
2	5	9	-1	0	1736654435036	f
2	5	13	-1	0	1736654448845	f
2	5	3	-1	0	1736654452251	f
2	5	23	0	0	1736654469613	f
2	5	22	-1	0	1736654473757	f
2	6	17	-1	0	1736654517943	f
2	6	2	0	0	1736654523240	f
2	6	11	0	0	1736654535314	f
2	6	5	-1	0	1736654537136	f
2	6	12	-1	0	1736654539724	f
2	6	3	-1	0	1736654541942	f
2	6	1	1	0	1736654544375	f
2	6	10	-1	0	1736654553893	f
2	6	21	-1	0	1736654557532	f
2	6	20	-1	0	1736654559611	f
2	6	6	1	0	1736654561269	f
2	7	17	-1	0	1736654561446	f
2	6	15	-1	0	1736654563427	f
2	7	6	1	0	1736654564424	f
2	6	9	-1	0	1736654565069	f
2	6	24	-1	0	1736654568421	f
2	7	2	0	0	1736654570771	f
2	6	22	1	0	1736654571465	f
2	7	20	-1	0	1736654572802	f
2	6	13	-1	0	1736654573404	f
2	6	25	-1	0	1736654574875	f
2	7	1	1	0	1736654574949	f
2	7	13	-1	0	1736654581232	f
2	7	5	-1	0	1736654583111	f
2	7	11	0	0	1736654591384	f
2	7	12	-1	0	1736654594003	f
2	7	3	-1	0	1736654597138	f
2	7	25	-1	0	1736654604021	f
2	7	21	-1	0	1736654611798	f
2	7	15	-1	0	1736654621570	f
2	75	40	-1	0	1736740427591	f
2	75	60	1	0	1736740467335	f
2	75	55	-1	0	1736740475149	f
2	75	45	-1	0	1736740503655	f
2	75	44	-1	0	1736740510675	f
2	75	20	-1	0	1736740515843	f
2	75	41	-1	0	1736740526376	f
2	75	36	-1	0	1736740530594	f
2	75	49	0	0	1736740535779	f
2	59	70	1	0	1736744748847	f
5	0	9	0	0	1736791538585	f
5	0	10	0	0	1736791545645	f
5	5	8	-1	0	1736794917687	f
5	11	5	-1	0	1736797835340	f
5	11	16	1	0	1736797861249	f
5	11	8	-1	0	1736797873337	f
5	11	6	-1	0	1736797894087	f
5	11	3	-1	0	1736797899025	f
5	11	1	-1	0	1736797903836	f
5	11	11	1	0	1736797908536	f
5	14	11	1	0	1736803951271	f
5	18	12	1	0	1736814069886	f
5	18	3	-1	0	1736814074549	f
2	78	27	-1	0	1736827385606	f
2	78	41	-1	0	1736827953153	f
2	81	70	-1	0	1736879935585	f
2	81	30	-1	0	1736879947566	f
2	81	61	0	0	1736879954853	f
2	82	17	-1	0	1736880708411	f
2	82	57	0	0	1736881039895	f
2	86	28	-1	0	1736882473263	f
2	86	15	-1	0	1736882497487	f
2	86	69	-1	0	1736882505754	f
2	86	50	-1	0	1736882515620	f
2	86	12	-1	0	1736882534081	f
2	86	23	1	0	1736882548800	f
2	87	39	-1	0	1736916436786	f
2	87	41	-1	0	1736917303880	f
2	87	48	0	0	1736917320825	f
2	87	49	-1	0	1736917331906	f
2	88	56	-1	0	1736994851897	f
2	89	38	-1	0	1737002092085	f
2	89	3	-1	0	1737002107577	f
2	90	72	0	0	1737041306019	f
2	90	43	1	0	1737041667957	f
2	90	36	1	0	1737041675124	f
2	91	34	-1	0	1737265258365	f
2	91	6	-1	0	1737265264085	f
2	91	30	-1	0	1737265278330	f
2	91	54	-1	0	1737265299052	f
2	91	23	-1	0	1737265315969	f
2	91	73	0	0	1737265332884	f
2	91	60	-1	0	1737265338269	f
3	7	7	-1	0	1737730363690	f
9	4	1	-1	0	1739378807957	f
12	0	16	-1	0	1739382229864	f
12	4	0	1	0	1739454493235	f
12	4	17	-1	0	1739454503027	f
12	28	19	-1	0	1739454999035	f
12	10	23	-1	0	1739455036634	f
12	11	19	1	0	1739455555952	f
12	21	28	1	0	1739455885661	f
12	5	8	1	0	1739456402079	f
9	11	36	-1	0	1739456937285	f
12	30	31	-1	0	1739457891204	f
12	34	3	1	0	1739458585976	f
12	2	2	-1	0	1739458592214	f
12	21	34	-1	0	1739459489009	f
12	2	1	-1	0	1739459573849	f
12	7	1	-1	0	1739459803237	f
12	38	33	-1	0	1739459805058	f
12	22	33	-1	0	1739459806581	f
12	7	34	1	0	1739459814205	f
12	7	29	1	0	1739459819266	f
12	10	1	-1	0	1739459820952	f
12	38	1	-1	0	1739459820992	f
12	7	2	-1	0	1739459832115	f
12	17	32	0	0	1739459832261	f
12	10	3	-1	0	1739459832567	f
12	27	19	-1	0	1739460085318	f
12	27	8	1	0	1739460115690	f
12	44	29	-1	0	1739460153860	f
12	44	6	-1	0	1739460169708	f
12	45	33	-1	0	1739460876211	f
12	49	23	-1	0	1739461933825	f
9	14	32	-1	0	1739462756572	f
9	14	33	-1	0	1739462764358	f
9	14	31	0	0	1739462768556	f
12	12	14	-1	0	1739463042685	f
12	12	25	-1	0	1739463046134	f
12	18	22	-1	0	1739463776316	f
12	18	20	-1	0	1739463784179	f
12	35	25	-1	0	1739464230075	f
12	56	7	0	0	1739464235636	f
12	29	2	-1	0	1739464235684	f
12	29	29	1	0	1739464244636	f
12	31	22	1	0	1739464273165	f
12	15	29	1	0	1739464297235	f
12	56	28	0	0	1739464297404	f
12	2	36	-1	0	1739464297566	f
12	5	18	-1	0	1739464906830	f
12	29	18	-1	0	1739465197750	f
12	49	26	-1	0	1739465949398	f
12	49	38	-1	0	1739465951868	f
12	49	18	-1	0	1739465964062	f
12	22	36	-1	0	1739465964149	f
12	27	26	-1	0	1739466333039	f
12	17	18	-1	0	1739466368645	f
12	3	40	-1	0	1739468189521	f
12	60	33	-1	0	1739596015393	f
12	60	25	-1	0	1739596049612	f
2	7	23	-1	0	1736654586999	f
2	7	22	0	0	1736654601614	f
2	7	9	-1	0	1736654605685	f
2	7	10	-1	0	1736654607051	f
2	7	24	0	0	1736654618416	f
2	8	15	-1	0	1736654654183	f
2	8	9	-1	0	1736654656664	f
2	8	1	1	0	1736654658624	f
2	8	3	-1	0	1736654660736	f
2	8	6	1	0	1736654662688	f
2	8	20	-1	0	1736654664159	f
2	8	5	-1	0	1736654665354	f
2	8	21	-1	0	1736654667561	f
2	8	2	-1	0	1736654670624	f
2	8	10	-1	0	1736654672389	f
2	8	25	-1	0	1736654673708	f
2	8	12	-1	0	1736654675141	f
2	8	23	-1	0	1736654676936	f
2	8	17	-1	0	1736654678308	f
2	8	24	-1	0	1736654681613	f
2	8	13	-1	0	1736654683212	f
2	8	11	-1	0	1736654684965	f
2	8	22	-1	0	1736654686206	f
2	9	2	0	0	1736654747999	f
2	9	23	-1	0	1736654758677	f
2	9	5	-1	0	1736654763396	f
2	9	22	1	0	1736654786325	f
2	9	17	0	0	1736654799152	f
2	9	10	-1	0	1736654807086	f
2	9	15	-1	0	1736654812305	f
2	9	21	-1	0	1736654818076	f
2	9	24	-1	0	1736654829037	f
2	9	1	0	0	1736654833505	f
2	9	12	1	0	1736654852894	f
2	10	9	-1	0	1736654854131	f
2	9	6	0	0	1736654856254	f
2	10	15	-1	0	1736654858297	f
2	10	6	0	0	1736654861114	f
2	9	25	-1	0	1736654863730	f
2	10	11	-1	0	1736654864298	f
2	10	1	0	0	1736654868556	f
2	10	2	0	0	1736654872487	f
2	9	9	0	0	1736654879187	f
2	10	20	0	0	1736654880325	f
2	9	3	1	0	1736654884218	f
2	10	24	0	0	1736654885445	f
2	9	13	1	0	1736654893501	f
2	10	12	-1	0	1736654896375	f
2	10	22	0	0	1736654899421	f
2	9	11	0	0	1736654901331	f
2	10	17	-1	0	1736654901886	f
2	10	3	-1	0	1736654909936	f
2	10	23	-1	0	1736654912188	f
2	10	10	-1	0	1736654915413	f
2	9	20	1	0	1736654917491	f
2	10	13	-1	0	1736654919374	f
2	10	25	-1	0	1736654922864	f
2	11	20	-1	0	1736654927215	f
2	11	1	1	0	1736654963533	f
2	11	2	1	0	1736654966364	f
2	11	5	-1	0	1736654968564	f
2	11	12	-1	0	1736654971155	f
2	11	6	1	0	1736654972952	f
2	11	21	-1	0	1736654976615	f
2	10	21	-1	0	1736654979519	f
2	10	5	-1	0	1736654982610	f
2	11	24	0	0	1736654991159	f
2	11	22	-1	0	1736655012782	f
2	11	15	-1	0	1736655015411	f
2	11	13	-1	0	1736655017621	f
2	11	10	-1	0	1736655022390	f
2	11	27	-1	0	1736655065841	f
2	11	23	-1	0	1736655079838	f
2	11	9	-1	0	1736655082239	f
2	11	25	-1	0	1736655084498	f
2	11	17	-1	0	1736655086360	f
2	11	3	-1	0	1736655089574	f
2	11	11	-1	0	1736655097908	f
2	10	28	-1	0	1736655163874	f
2	12	1	1	0	1736655204388	f
2	10	27	-1	0	1736655204712	f
2	12	6	1	0	1736655206583	f
2	12	2	-1	0	1736655218050	f
2	12	23	0	0	1736655227728	f
2	12	15	-1	0	1736655233091	f
2	12	22	-1	0	1736655242428	f
2	12	5	-1	0	1736655247304	f
2	12	27	-1	0	1736655256522	f
2	12	21	-1	0	1736655265814	f
2	12	13	-1	0	1736655270951	f
2	12	28	-1	0	1736655282742	f
2	13	24	0	0	1736655285270	f
2	13	6	-1	0	1736655287187	f
2	13	1	1	0	1736655290276	f
2	12	20	-1	0	1736655294865	f
2	13	2	0	0	1736655294943	f
2	12	9	-1	0	1736655296951	f
2	13	28	-1	0	1736655298265	f
2	13	13	-1	0	1736655300492	f
2	13	25	-1	0	1736655302429	f
2	13	12	-1	0	1736655304269	f
2	13	20	-1	0	1736655306685	f
2	13	5	-1	0	1736655308282	f
2	13	17	-1	0	1736655309871	f
2	13	27	-1	0	1736655312813	f
2	12	25	-1	0	1736655314430	f
2	13	9	-1	0	1736655316501	f
2	12	10	-1	0	1736655317448	f
2	13	10	-1	0	1736655318484	f
2	13	15	-1	0	1736655320398	f
2	12	17	-1	0	1736655321405	f
2	13	23	-1	0	1736655323697	f
2	13	21	-1	0	1736655325848	f
2	13	11	0	0	1736655329937	f
2	13	22	1	0	1736655334755	f
2	12	12	0	0	1736655335219	f
2	12	3	-1	0	1736655338421	f
2	13	3	-1	0	1736655339360	f
2	12	24	1	0	1736655350318	f
2	12	11	-1	0	1736655354213	f
2	14	28	-1	0	1736655354541	f
2	14	13	-1	0	1736655358501	f
2	14	1	1	0	1736655363787	f
2	14	6	1	0	1736655365778	f
2	14	2	1	0	1736655373550	f
2	14	5	-1	0	1736655375803	f
2	14	21	-1	0	1736655381260	f
2	14	27	-1	0	1736655405705	f
2	14	20	-1	0	1736655410626	f
2	14	23	-1	0	1736655429376	f
2	14	12	-1	0	1736655431642	f
2	13	29	-1	0	1736655435925	f
2	12	30	-1	0	1736655444571	f
2	12	29	1	0	1736655464263	f
2	13	31	-1	0	1736655495157	f
2	13	30	1	0	1736655506625	f
2	14	24	1	0	1736655533911	f
2	14	30	1	0	1736655540684	f
2	13	32	-1	0	1736655563299	f
2	14	29	-1	0	1736655592279	f
2	14	10	-1	0	1736655595042	f
2	75	42	-1	0	1736740430807	f
2	75	39	0	0	1736740442110	f
2	75	56	-1	0	1736740449545	f
2	59	72	-1	0	1736744752468	f
5	0	11	0	0	1736791566273	f
5	5	7	-1	0	1736794935269	f
5	5	4	-1	0	1736794946965	f
5	5	14	1	0	1736794960367	f
5	12	12	0	0	1736801499036	f
5	12	10	1	0	1736801502766	f
5	15	1	1	0	1736804657932	f
5	15	10	1	0	1736804662893	f
5	15	18	0	0	1736804704337	f
5	15	17	-1	0	1736804710102	f
5	18	9	-1	0	1736814080859	f
5	18	6	-1	0	1736814093784	f
2	78	62	0	0	1736827396977	f
2	78	34	0	0	1736827444195	f
2	78	40	-1	0	1736827474317	f
2	78	44	-1	0	1736827484951	f
2	78	28	0	0	1736827526564	f
2	78	57	-1	0	1736827957674	f
2	82	6	-1	0	1736880375756	f
2	84	2	-1	0	1736880731804	f
2	84	44	-1	0	1736880744272	f
2	84	3	-1	0	1736880749779	f
2	85	1	1	0	1736881628303	f
2	85	39	-1	0	1736881653201	f
2	85	30	-1	0	1736881659352	f
2	85	21	-1	0	1736881669299	f
2	85	29	-1	0	1736881676917	f
2	85	9	-1	0	1736881712126	f
2	85	34	-1	0	1736881720934	f
2	86	3	-1	0	1736882477130	f
2	86	54	-1	0	1736882483861	f
2	86	58	-1	0	1736882509708	f
2	86	24	0	0	1736882522169	f
2	86	35	-1	0	1736882531880	f
2	86	36	-1	0	1736882539343	f
2	86	11	-1	0	1736882543022	f
2	86	20	1	0	1736882551916	f
2	86	29	1	0	1736882580338	f
2	86	56	0	0	1736882592305	f
2	86	31	-1	0	1736882596466	f
2	86	66	-1	0	1736882602994	f
2	86	44	-1	0	1736882607162	f
2	86	61	-1	0	1736882612593	f
2	86	62	-1	0	1736882626172	f
2	86	57	1	0	1736882632942	f
2	87	67	-1	0	1736916454103	f
2	87	9	-1	0	1736917306275	f
2	88	13	-1	0	1736994858320	f
2	88	72	-1	0	1736994872837	f
2	89	49	1	0	1737002100519	f
2	90	21	-1	0	1737041311263	f
2	90	67	1	0	1737041698572	f
2	91	46	-1	0	1737265267684	f
2	91	15	-1	0	1737265272225	f
2	91	58	-1	0	1737265275174	f
2	91	3	-1	0	1737265286526	f
2	91	47	0	0	1737265296108	f
2	91	33	-1	0	1737265307706	f
2	91	44	-1	0	1737265319366	f
2	91	48	1	0	1737265328679	f
3	7	8	-1	0	1737730617977	f
9	6	3	-1	0	1739381298585	f
12	0	17	-1	0	1739382236501	f
12	0	18	-1	0	1739382252112	f
9	8	30	0	0	1739454530118	f
12	28	0	1	0	1739455030241	f
12	11	28	1	0	1739455563063	f
12	21	29	0	0	1739455894307	f
12	21	19	-1	0	1739455902436	f
12	21	6	0	0	1739455925705	f
12	12	29	-1	0	1739456411871	f
9	8	36	1	0	1739456947959	f
12	30	7	1	0	1739457907548	f
12	2	3	-1	0	1739458588965	f
12	2	32	-1	0	1739458598427	f
12	2	13	-1	0	1739459578426	f
12	4	1	-1	0	1739459597931	f
12	10	33	1	0	1739459815069	f
12	7	33	-1	0	1739459829265	f
12	41	33	-1	0	1739459833058	f
12	41	13	-1	0	1739459837207	f
12	17	3	1	0	1739459839887	f
12	38	32	0	0	1739459842596	f
12	41	31	-1	0	1739459844353	f
12	27	27	1	0	1739460098524	f
12	27	7	0	0	1739460107444	f
12	44	2	-1	0	1739460127168	f
12	45	17	0	0	1739460899141	f
12	49	32	0	0	1739461946507	f
9	14	36	-1	0	1739462759041	f
9	14	29	0	0	1739462774076	f
12	54	32	-1	0	1739463054926	f
12	5	20	-1	0	1739463844513	f
12	20	20	-1	0	1739464234763	f
12	20	36	-1	0	1739464242051	f
12	35	14	-1	0	1739464249554	f
12	29	3	-1	0	1739464258836	f
12	15	32	-1	0	1739464270028	f
12	15	1	0	0	1739464286059	f
12	56	32	0	0	1739464291032	f
12	5	21	-1	0	1739464912417	f
12	11	37	-1	0	1739464912682	f
12	16	21	-1	0	1739464924603	f
12	2	37	0	0	1739464942283	f
12	53	8	1	0	1739465241731	f
12	53	28	1	0	1739465245765	f
12	15	37	-1	0	1739465956037	f
12	16	38	-1	0	1739465961933	f
12	16	26	-1	0	1739465969826	f
12	59	14	-1	0	1739465975550	f
12	35	26	-1	0	1739466347043	f
12	0	41	0	0	1739468200953	f
12	60	14	0	0	1739596024232	f
2	14	9	-1	0	1736655536674	f
2	14	25	-1	0	1736655543945	f
2	14	31	-1	0	1736655586240	f
2	14	15	-1	0	1736655639130	f
2	14	32	-1	0	1736655647372	f
2	14	22	1	0	1736655660068	f
2	14	17	-1	0	1736655662637	f
2	14	11	0	0	1736655668464	f
2	14	3	-1	0	1736655671945	f
2	0	29	-1	0	1736655942814	f
2	0	30	1	0	1736655946778	f
2	0	28	-1	0	1736655952130	f
2	0	27	-1	0	1736655960445	f
2	0	31	-1	0	1736655964126	f
2	0	32	-1	0	1736655968026	f
2	15	2	-1	0	1736656291010	f
2	15	32	0	0	1736656300973	f
2	15	20	0	0	1736656309560	f
2	15	1	0	0	1736656312479	f
2	15	29	-1	0	1736656317166	f
2	15	30	-1	0	1736656322108	f
2	15	6	1	0	1736656327295	f
2	15	11	-1	0	1736656330061	f
2	15	27	0	0	1736656337375	f
2	16	15	-1	0	1736656341096	f
2	16	1	0	0	1736656348157	f
2	16	6	1	0	1736656355644	f
2	15	31	-1	0	1736656361596	f
2	16	31	-1	0	1736656363282	f
2	16	21	-1	0	1736656366380	f
2	16	2	1	0	1736656368896	f
2	15	28	0	0	1736656370543	f
2	15	13	-1	0	1736656373097	f
2	15	3	-1	0	1736656376163	f
2	15	12	-1	0	1736656378510	f
2	15	9	-1	0	1736656380804	f
2	16	30	1	0	1736656381491	f
2	15	5	0	0	1736656384097	f
2	16	29	-1	0	1736656385903	f
2	15	23	0	0	1736656391154	f
2	16	20	-1	0	1736656394700	f
2	15	22	0	0	1736656395952	f
2	16	28	0	0	1736656404033	f
2	16	24	1	0	1736656417118	f
2	16	12	-1	0	1736656419656	f
2	15	17	-1	0	1736656424449	f
2	16	32	-1	0	1736656427453	f
2	15	25	-1	0	1736656429507	f
2	16	9	-1	0	1736656429722	f
2	15	15	-1	0	1736656432524	f
2	16	27	-1	0	1736656432994	f
2	15	10	-1	0	1736656436023	f
2	16	13	-1	0	1736656436395	f
2	15	21	-1	0	1736656439407	f
2	16	17	-1	0	1736656441296	f
2	15	24	0	0	1736656445837	f
2	16	23	-1	0	1736656450185	f
2	16	25	-1	0	1736656455333	f
2	16	10	-1	0	1736656457108	f
2	16	22	0	0	1736656461066	f
2	16	11	-1	0	1736656464115	f
2	16	5	0	0	1736656467519	f
2	16	3	-1	0	1736656471574	f
2	15	33	-1	0	1736656492202	f
2	13	33	0	0	1736656540549	f
2	15	34	-1	0	1736656618759	f
2	13	34	-1	0	1736656674264	f
2	15	35	-1	0	1736656776321	f
2	17	32	-1	0	1736656787614	f
2	17	1	0	0	1736656797031	f
2	17	31	-1	0	1736656802790	f
2	17	34	-1	0	1736656809735	f
2	17	27	-1	0	1736656827615	f
2	17	30	1	0	1736656833865	f
2	17	20	-1	0	1736656839875	f
2	17	28	-1	0	1736656846200	f
2	17	6	-1	0	1736656869129	f
2	17	2	-1	0	1736656873322	f
2	13	35	-1	0	1736656886636	f
2	17	33	-1	0	1736656888755	f
2	17	29	-1	0	1736656896382	f
2	17	13	-1	0	1736656908940	f
2	17	35	-1	0	1736656913668	f
2	17	17	-1	0	1736656926585	f
2	17	12	-1	0	1736656931131	f
2	17	9	-1	0	1736656934515	f
2	17	10	-1	0	1736656938415	f
2	17	11	-1	0	1736656943154	f
2	17	3	-1	0	1736656946671	f
2	17	21	-1	0	1736656950838	f
2	17	5	-1	0	1736656954045	f
2	17	22	-1	0	1736656959787	f
2	17	15	-1	0	1736656964188	f
2	17	23	-1	0	1736656966953	f
2	17	25	-1	0	1736656970097	f
2	17	24	0	0	1736656978206	f
2	18	29	-1	0	1736657017459	f
2	0	36	0	0	1736657019908	f
2	18	30	1	0	1736657023150	f
2	19	27	0	0	1736657028359	f
2	18	33	-1	0	1736657032336	f
2	19	1	1	0	1736657035693	f
2	19	12	-1	0	1736657041545	f
2	18	32	0	0	1736657051810	f
2	18	10	-1	0	1736657054845	f
2	18	6	1	0	1736657057554	f
2	18	1	1	0	1736657059531	f
2	18	12	-1	0	1736657062252	f
2	19	32	0	0	1736657062417	f
2	18	34	-1	0	1736657066998	f
2	19	30	-1	0	1736657070943	f
2	19	2	1	0	1736657081916	f
2	19	6	1	0	1736657084528	f
2	17	36	-1	0	1736657085511	f
2	19	28	-1	0	1736657093683	f
2	19	29	0	0	1736657104473	f
2	0	37	0	0	1736657112070	f
2	19	36	-1	0	1736657112790	f
2	19	34	-1	0	1736657121237	f
2	19	5	-1	0	1736657129636	f
2	17	37	-1	0	1736657129996	f
2	19	20	-1	0	1736657136437	f
2	19	22	-1	0	1736657152188	f
2	20	2	0	0	1736657159900	f
2	19	37	0	0	1736657168402	f
2	21	2	1	0	1736657170196	f
2	21	6	1	0	1736657172339	f
2	21	5	-1	0	1736657175195	f
2	21	30	1	0	1736657180554	f
2	20	15	-1	0	1736657183749	f
2	22	13	-1	0	1736657185873	f
2	21	13	0	0	1736657187188	f
2	21	1	1	0	1736657189022	f
2	22	30	-1	0	1736657193515	f
2	20	32	-1	0	1736657194251	f
2	21	22	1	0	1736657194284	f
2	21	31	1	0	1736657199231	f
2	20	35	-1	0	1736657202091	f
2	21	28	-1	0	1736657213666	f
2	21	33	-1	0	1736657217900	f
2	23	1	1	0	1736657221948	f
2	24	20	-1	0	1736657224566	f
2	22	34	-1	0	1736657224789	f
2	20	5	-1	0	1736657232097	f
2	23	21	-1	0	1736657236272	f
2	22	27	1	0	1736657240279	f
2	13	36	-1	0	1736657242389	f
2	23	29	-1	0	1736657244838	f
2	20	31	-1	0	1736657247578	f
2	23	33	-1	0	1736657250443	f
2	23	23	-1	0	1736657259073	f
2	22	33	-1	0	1736657269856	f
2	20	12	-1	0	1736657272787	f
2	20	36	-1	0	1736657278620	f
2	24	5	-1	0	1736657284094	f
2	20	3	-1	0	1736657305934	f
2	75	52	-1	0	1736740433871	f
2	75	27	-1	0	1736740445425	f
2	75	5	-1	0	1736740452180	f
2	59	73	-1	0	1736744822886	f
5	0	12	0	0	1736791579161	f
5	5	15	-1	0	1736794939538	f
5	5	16	-1	0	1736794973365	f
5	0	18	-1	0	1736802170245	f
5	15	9	1	0	1736804661413	f
5	15	11	1	0	1736804674795	f
5	15	14	-1	0	1736804688920	f
5	15	8	-1	0	1736804697558	f
5	18	21	1	0	1736814082623	f
5	18	11	1	0	1736814100229	f
5	18	14	1	0	1736814119704	f
2	78	73	-1	0	1736827403724	f
2	78	36	-1	0	1736827417493	f
2	11	73	-1	0	1736879439645	f
2	11	60	-1	0	1736879449634	f
2	11	59	0	0	1736879500251	f
2	82	2	-1	0	1736880384219	f
2	82	33	1	0	1736880406151	f
2	82	44	0	0	1736880734490	f
2	84	45	-1	0	1736880737308	f
2	84	1	0	0	1736880741479	f
2	84	6	-1	0	1736880747018	f
2	84	20	-1	0	1736880752552	f
2	82	51	1	0	1736880756854	f
2	84	53	-1	0	1736880757302	f
2	85	2	-1	0	1736881632456	f
2	85	38	1	0	1736881641968	f
2	85	6	-1	0	1736881648255	f
2	85	70	-1	0	1736881657313	f
2	85	69	-1	0	1736881663197	f
2	85	67	-1	0	1736881681689	f
2	86	21	-1	0	1736882486907	f
2	86	72	-1	0	1736882495016	f
2	86	51	1	0	1736882500514	f
2	86	64	1	0	1736882526497	f
2	86	22	-1	0	1736882541115	f
2	86	60	-1	0	1736882567609	f
2	86	63	-1	0	1736882583717	f
2	86	17	-1	0	1736882605116	f
2	86	52	-1	0	1736882609494	f
2	86	46	-1	0	1736882617797	f
2	87	40	-1	0	1736916466680	f
2	87	72	-1	0	1736916481774	f
2	87	59	0	0	1736916496530	f
2	87	25	0	0	1736917314076	f
2	87	50	-1	0	1736917326340	f
2	87	62	0	0	1736917345348	f
2	88	28	-1	0	1736994868606	f
2	88	73	0	0	1736994887793	f
2	89	70	-1	0	1737002104504	f
2	90	73	0	0	1737041340495	f
2	90	25	-1	0	1737041344798	f
2	90	70	-1	0	1737041349462	f
2	90	42	0	0	1737041712900	f
2	90	15	-1	0	1737041716312	f
2	91	13	-1	0	1737265269687	f
3	7	4	1	0	1737730619993	f
9	6	4	-1	0	1739381326667	f
12	0	19	-1	0	1739382266514	f
12	0	22	-1	0	1739382290352	f
12	0	23	-1	0	1739382299820	f
12	0	25	-1	0	1739382318584	f
12	6	4	-1	0	1739454566000	f
12	6	17	-1	0	1739454570641	f
12	2	19	-1	0	1739455113344	f
12	11	23	1	0	1739455569207	f
12	2	28	1	0	1739455574898	f
12	29	28	1	0	1739455585276	f
12	21	27	1	0	1739455907860	f
12	12	28	1	0	1739456420384	f
12	12	11	1	0	1739456424980	f
12	12	8	1	0	1739456430078	f
12	20	6	-1	0	1739456441851	f
12	32	29	1	0	1739456444985	f
12	20	30	-1	0	1739456455882	f
9	8	37	-1	0	1739457013474	f
12	0	32	0	0	1739457992791	f
12	2	33	-1	0	1739458623284	f
12	4	13	-1	0	1739459595407	f
12	10	2	-1	0	1739459824214	f
12	36	11	1	0	1739459827021	f
12	20	7	-1	0	1739459829456	f
12	36	27	-1	0	1739459830407	f
12	38	29	-1	0	1739459830714	f
12	7	11	1	0	1739459844644	f
12	17	29	1	0	1739459846900	f
12	10	32	-1	0	1739459847177	f
12	17	1	1	0	1739459852368	f
12	17	33	0	0	1739459876012	f
12	44	33	-1	0	1739460144311	f
12	44	3	-1	0	1739460158472	f
12	45	0	0	0	1739460938488	f
12	45	2	-1	0	1739460992923	f
12	49	33	-1	0	1739461950091	f
12	49	1	-1	0	1739461953946	f
9	14	35	-1	0	1739462771948	f
9	14	38	-1	0	1739462775383	f
9	14	30	0	0	1739462776620	f
12	0	35	0	0	1739463187100	f
12	12	22	-1	0	1739463856612	f
12	56	20	-1	0	1739463951054	f
12	56	27	1	0	1739463974233	f
12	56	13	-1	0	1739463978753	f
12	5	36	-1	0	1739464254424	f
12	29	20	1	0	1739464274200	f
12	29	33	-1	0	1739464287884	f
12	29	30	1	0	1739464308616	f
12	2	18	-1	0	1739464912637	f
12	8	18	-1	0	1739465394753	f
12	59	35	-1	0	1739465989371	f
12	59	17	-1	0	1739465993781	f
12	59	23	-1	0	1739466000635	f
2	22	2	-1	0	1736657200658	f
2	21	34	-1	0	1736657205516	f
2	22	6	1	0	1736657207232	f
2	21	29	0	0	1736657210200	f
2	24	30	1	0	1736657211869	f
2	22	1	0	0	1736657216599	f
2	23	31	-1	0	1736657218231	f
2	75	10	-1	0	1736740470535	f
2	75	51	0	0	1736740480601	f
2	75	62	0	0	1736740507988	f
2	75	11	0	0	1736740520276	f
2	75	50	0	0	1736740538772	f
2	59	69	1	0	1736744827997	f
5	1	12	-1	0	1736794349435	f
5	1	9	1	0	1736794351658	f
5	1	6	-1	0	1736794358618	f
5	5	3	0	0	1736794941857	f
5	5	2	0	0	1736794948816	f
5	0	16	-1	0	1736802171572	f
5	0	20	-1	0	1736802179016	f
5	15	2	1	0	1736804665236	f
5	15	12	-1	0	1736804671478	f
5	15	3	1	0	1736804673594	f
5	15	16	1	0	1736804676447	f
5	15	7	-1	0	1736804693271	f
5	15	20	-1	0	1736804715325	f
5	18	2	-1	0	1736814095796	f
5	18	16	1	0	1736814098743	f
5	18	1	1	0	1736814102209	f
5	18	22	-1	0	1736814123257	f
2	78	17	-1	0	1736827426386	f
2	78	67	0	0	1736827433076	f
2	78	30	1	0	1736827448279	f
2	11	67	-1	0	1736879445376	f
2	11	64	-1	0	1736879453796	f
2	11	65	-1	0	1736879462092	f
2	11	72	1	0	1736879469872	f
2	11	70	-1	0	1736879486997	f
2	11	63	-1	0	1736879490293	f
2	11	68	-1	0	1736879506976	f
2	82	64	-1	0	1736880391530	f
2	82	11	0	0	1736880767259	f
2	82	55	-1	0	1736880771462	f
2	85	73	-1	0	1736881636142	f
2	85	63	-1	0	1736881645749	f
2	85	35	-1	0	1736881665546	f
2	85	54	-1	0	1736881671519	f
2	85	27	-1	0	1736881697042	f
2	85	17	-1	0	1736881723194	f
2	85	65	1	0	1736881749490	f
2	85	10	-1	0	1736881757282	f
2	85	68	-1	0	1736881769562	f
2	85	71	-1	0	1736881780882	f
2	85	51	1	0	1736881790161	f
2	85	64	1	0	1736881820982	f
2	85	28	-1	0	1736881835312	f
2	85	22	-1	0	1736881849565	f
2	86	55	-1	0	1736882570458	f
2	86	39	-1	0	1736882575784	f
2	86	43	-1	0	1736882581821	f
2	86	9	-1	0	1736882597891	f
2	86	5	1	0	1736882621847	f
2	86	49	-1	0	1736882629392	f
2	87	69	1	0	1736916474144	f
2	87	38	0	0	1736916488804	f
2	87	24	0	0	1736917357769	f
2	88	69	-1	0	1736994900071	f
2	89	73	-1	0	1737002340034	f
2	90	30	-1	0	1737041358394	f
2	90	59	-1	0	1737041721872	f
2	91	42	-1	0	1737265370945	f
2	91	72	0	0	1737265377818	f
2	91	59	-1	0	1737265386505	f
2	91	5	-1	0	1737265402132	f
2	91	40	-1	0	1737265410564	f
2	91	69	-1	0	1737265446490	f
2	91	31	-1	0	1737265467243	f
2	91	39	-1	0	1737265490409	f
2	91	17	-1	0	1737265511551	f
2	91	41	1	0	1737265524669	f
3	7	5	-1	0	1737731277838	f
9	6	5	-1	0	1739381383056	f
12	0	20	-1	0	1739382273234	f
12	6	0	1	0	1739454576000	f
12	7	17	-1	0	1739454612405	f
12	2	27	1	0	1739455119576	f
12	22	28	1	0	1739455574330	f
12	11	27	1	0	1739455577064	f
12	21	23	-1	0	1739455914322	f
12	21	8	1	0	1739455921439	f
12	12	6	1	0	1739456434054	f
12	32	17	-1	0	1739456456231	f
12	22	29	0	0	1739456460900	f
12	32	23	-1	0	1739456464849	f
9	11	37	-1	0	1739457076833	f
12	5	32	1	0	1739458081709	f
12	18	32	0	0	1739458634363	f
12	18	3	-1	0	1739458636826	f
12	18	2	-1	0	1739458638831	f
12	5	33	-1	0	1739458676648	f
12	4	33	-1	0	1739458678357	f
12	12	33	-1	0	1739458699998	f
12	35	3	-1	0	1739459642307	f
12	13	13	-1	0	1739459677945	f
12	17	2	-1	0	1739459835188	f
12	10	13	-1	0	1739459839774	f
12	38	4	0	0	1739459848004	f
12	38	7	1	0	1739459855816	f
12	27	29	1	0	1739459883722	f
12	17	13	-1	0	1739459887186	f
12	16	27	-1	0	1739459887453	f
12	42	33	0	0	1739459887811	f
12	41	27	-1	0	1739459888769	f
12	27	6	-1	0	1739459890364	f
12	27	34	1	0	1739459895964	f
12	40	33	1	0	1739459898351	f
12	35	28	0	0	1739459900007	f
12	44	19	0	0	1739460176654	f
12	44	11	-1	0	1739460217775	f
12	44	32	1	0	1739460230622	f
12	45	6	-1	0	1739460960367	f
12	45	23	-1	0	1739460982979	f
12	45	4	-1	0	1739461045997	f
12	49	11	1	0	1739461968778	f
12	49	28	0	0	1739461983286	f
12	49	8	1	0	1739461986884	f
12	34	14	0	0	1739462812254	f
12	31	35	-1	0	1739463200655	f
12	2	35	-1	0	1739463202239	f
12	5	25	-1	0	1739463255107	f
12	12	20	-1	0	1739463862455	f
12	56	33	-1	0	1739464257100	f
12	15	8	-1	0	1739464276538	f
12	31	36	-1	0	1739464277282	f
12	5	37	-1	0	1739464916497	f
12	16	18	-1	0	1739464917257	f
2	20	22	0	0	1736657206028	f
2	23	34	-1	0	1736657209910	f
2	22	35	-1	0	1736657212044	f
2	20	6	1	0	1736657218450	f
2	24	6	1	0	1736657220088	f
2	20	28	-1	0	1736657222536	f
2	24	1	1	0	1736657227318	f
2	23	30	1	0	1736657228197	f
2	20	34	-1	0	1736657229439	f
2	23	10	-1	0	1736657232247	f
2	20	30	1	0	1736657237134	f
2	24	21	-1	0	1736657237480	f
2	23	9	-1	0	1736657238676	f
2	20	13	-1	0	1736657239871	f
2	23	6	1	0	1736657240383	f
2	20	1	1	0	1736657242260	f
2	24	23	-1	0	1736657243438	f
2	22	25	-1	0	1736657244544	f
2	13	37	-1	0	1736657246516	f
2	22	12	-1	0	1736657247835	f
2	24	2	1	0	1736657247922	f
2	20	20	-1	0	1736657251596	f
2	20	27	-1	0	1736657255562	f
2	22	29	-1	0	1736657256359	f
2	24	33	0	0	1736657258777	f
2	22	36	-1	0	1736657262295	f
2	24	36	-1	0	1736657264844	f
2	20	21	-1	0	1736657270103	f
2	23	35	-1	0	1736657271889	f
2	22	20	1	0	1736657273285	f
2	24	28	0	0	1736657275337	f
2	23	2	1	0	1736657277207	f
2	24	15	-1	0	1736657280310	f
2	23	20	-1	0	1736657282261	f
2	20	37	-1	0	1736657284109	f
2	23	17	-1	0	1736657284399	f
2	20	10	-1	0	1736657287397	f
2	20	33	0	0	1736657296856	f
2	20	9	-1	0	1736657299202	f
2	23	37	-1	0	1736657301432	f
2	23	15	-1	0	1736657304760	f
2	23	12	-1	0	1736657307657	f
2	24	35	-1	0	1736657309082	f
2	20	29	-1	0	1736657311738	f
2	23	36	-1	0	1736657313749	f
2	24	29	-1	0	1736657315950	f
2	20	25	-1	0	1736657316580	f
2	23	3	-1	0	1736657317136	f
2	20	17	-1	0	1736657320458	f
2	24	25	-1	0	1736657321956	f
2	20	23	-1	0	1736657325164	f
2	20	24	-1	0	1736657330827	f
2	24	34	0	0	1736657331496	f
2	23	22	-1	0	1736657332086	f
2	20	11	-1	0	1736657337531	f
2	24	11	0	0	1736657338557	f
2	23	13	-1	0	1736657342211	f
2	24	31	-1	0	1736657344975	f
2	23	28	-1	0	1736657346008	f
2	23	27	-1	0	1736657351756	f
2	24	24	0	0	1736657352495	f
2	24	10	0	0	1736657357463	f
2	23	11	-1	0	1736657358443	f
2	23	25	-1	0	1736657362853	f
2	0	38	0	0	1736657364890	f
2	23	32	-1	0	1736657371773	f
2	24	12	0	0	1736657375384	f
2	23	38	-1	0	1736657380623	f
2	24	37	0	0	1736657382439	f
2	24	13	-1	0	1736657386559	f
2	23	5	-1	0	1736657389048	f
2	24	27	-1	0	1736657391665	f
2	24	3	-1	0	1736657395887	f
2	23	24	1	0	1736657396423	f
2	24	32	-1	0	1736657408551	f
2	24	38	1	0	1736657413556	f
2	24	22	0	0	1736657419514	f
2	24	9	0	0	1736657424242	f
2	24	17	-1	0	1736657428295	f
2	0	39	0	0	1736657461734	f
2	0	33	-1	0	1736657497967	f
2	0	35	-1	0	1736657501287	f
2	0	34	-1	0	1736657504401	f
2	15	36	0	0	1736657545419	f
2	15	37	0	0	1736657568236	f
2	15	39	-1	0	1736657575799	f
2	25	1	1	0	1736657594934	f
2	25	2	-1	0	1736657597129	f
2	25	6	1	0	1736657599291	f
2	25	28	-1	0	1736657603346	f
2	25	11	0	0	1736657610679	f
2	25	34	-1	0	1736657614086	f
2	25	39	-1	0	1736657617411	f
2	25	35	-1	0	1736657621357	f
2	25	30	-1	0	1736657625606	f
2	25	20	-1	0	1736657628072	f
2	25	29	-1	0	1736657632526	f
2	25	31	-1	0	1736657635237	f
2	25	22	-1	0	1736657639293	f
2	25	36	-1	0	1736657641297	f
2	25	5	-1	0	1736657647507	f
2	25	13	-1	0	1736657649209	f
2	25	12	-1	0	1736657651255	f
2	25	33	-1	0	1736657654088	f
2	25	10	-1	0	1736657656245	f
2	25	37	-1	0	1736657661957	f
2	25	32	-1	0	1736657682928	f
2	25	27	-1	0	1736657686467	f
2	25	21	-1	0	1736657689104	f
2	25	23	-1	0	1736657691391	f
2	25	17	-1	0	1736657693114	f
2	25	15	-1	0	1736657695146	f
2	25	38	-1	0	1736657696799	f
2	25	9	-1	0	1736657699090	f
2	25	25	-1	0	1736657700905	f
2	25	3	-1	0	1736657703591	f
2	25	24	1	0	1736657707646	f
2	26	31	-1	0	1736657746112	f
2	26	1	0	0	1736657766973	f
2	26	2	0	0	1736657771384	f
2	26	30	-1	0	1736657784574	f
2	27	38	1	0	1736657793193	f
2	26	29	-1	0	1736657801372	f
2	26	6	-1	0	1736657803271	f
2	27	31	0	0	1736657806065	f
2	27	1	1	0	1736657808318	f
2	26	15	-1	0	1736657813040	f
2	27	6	1	0	1736657814631	f
2	26	36	-1	0	1736657817140	f
2	27	2	1	0	1736657818573	f
2	27	9	-1	0	1736657828001	f
2	28	6	1	0	1736657847075	f
2	27	39	1	0	1736657847975	f
2	28	1	1	0	1736657849635	f
2	27	27	-1	0	1736657856810	f
2	27	30	1	0	1736657861762	f
2	28	2	1	0	1736657863255	f
2	27	10	-1	0	1736657865012	f
2	28	3	-1	0	1736657870892	f
2	28	38	1	0	1736657878843	f
2	28	39	1	0	1736657881647	f
2	75	43	-1	0	1736740477651	f
2	75	32	0	0	1736740486788	f
4	0	0	0	0	1736754941030	f
5	1	11	1	0	1736794360564	f
5	1	4	-1	0	1736794406921	f
5	1	7	-1	0	1736794414061	f
5	5	5	1	0	1736794963053	f
5	0	19	-1	0	1736802184856	f
5	15	13	-1	0	1736804721761	f
5	18	20	-1	0	1736814126799	f
2	78	58	0	0	1736827463936	f
2	78	45	-1	0	1736827499162	f
2	78	54	-1	0	1736827518134	f
2	78	59	0	0	1736827533095	f
2	78	11	0	0	1736827555332	f
2	11	69	-1	0	1736879457762	f
2	11	61	1	0	1736879466283	f
2	11	71	0	0	1736879477764	f
2	11	66	-1	0	1736879503041	f
2	82	21	0	0	1736880437861	f
2	82	42	-1	0	1736880449540	f
2	83	13	-1	0	1736880480014	f
2	83	30	0	0	1736880487035	f
2	82	52	0	0	1736880785062	f
2	85	25	-1	0	1736881701125	f
2	85	56	-1	0	1736881710146	f
2	85	3	-1	0	1736881715942	f
2	87	1	1	0	1736916054141	f
2	87	45	0	0	1736916519361	f
2	87	57	0	0	1736917361935	f
2	88	47	0	0	1736994915127	f
2	89	44	-1	0	1737002342331	f
2	89	20	1	0	1737002353121	f
2	90	10	-1	0	1737041363349	f
2	90	71	-1	0	1737041734341	f
2	91	67	-1	0	1737265374894	f
2	91	53	-1	0	1737265380822	f
2	91	24	1	0	1737265398212	f
2	91	27	-1	0	1737265404935	f
2	91	37	0	0	1737265419685	f
2	91	12	-1	0	1737265429110	f
2	91	66	-1	0	1737265441153	f
2	91	10	-1	0	1737265448799	f
2	91	29	-1	0	1737265459772	f
2	91	62	0	0	1737265477875	f
3	7	1	1	0	1737731278888	f
3	7	6	1	0	1737731284723	f
9	6	6	-1	0	1739381406840	f
12	0	21	-1	0	1739382283575	f
12	0	24	-1	0	1739382306539	f
12	8	17	-1	0	1739454612443	f
12	8	0	1	0	1739454626336	f
12	2	23	-1	0	1739455135908	f
12	22	19	0	0	1739455582174	f
12	22	27	1	0	1739455589833	f
12	29	23	-1	0	1739455597252	f
12	11	6	1	0	1739455959399	f
12	12	19	-1	0	1739456444347	f
12	12	30	-1	0	1739456458872	f
12	18	27	1	0	1739457513110	f
12	2	7	-1	0	1739457523513	f
12	18	7	-1	0	1739457532375	f
12	12	7	-1	0	1739458094051	f
12	21	32	1	0	1739458107533	f
12	21	11	1	0	1739458119855	f
12	34	30	1	0	1739458134536	f
12	18	33	-1	0	1739458642202	f
12	34	0	-1	0	1739458655658	f
12	12	3	-1	0	1739458704175	f
12	35	1	-1	0	1739459648841	f
12	35	2	-1	0	1739459658313	f
12	13	29	-1	0	1739459700555	f
12	7	23	-1	0	1739459856274	f
12	41	6	-1	0	1739459856905	f
12	17	31	-1	0	1739459857139	f
12	38	19	-1	0	1739459860967	f
12	17	30	0	0	1739459871768	f
12	38	30	-1	0	1739459875728	f
12	14	32	0	0	1739459880117	f
12	30	34	-1	0	1739459894071	f
12	35	30	1	0	1739459895166	f
12	43	2	-1	0	1739459904680	f
12	43	6	-1	0	1739459907655	f
12	8	31	-1	0	1739459911199	f
12	40	31	1	0	1739459912941	f
12	16	29	0	0	1739459913491	f
12	14	13	-1	0	1739459926655	f
12	17	11	0	0	1739459930574	f
12	43	31	-1	0	1739459930764	f
12	16	34	1	0	1739459934009	f
12	14	3	0	0	1739459937467	f
12	40	6	-1	0	1739459941461	f
12	43	30	1	0	1739459944181	f
12	43	29	-1	0	1739459946819	f
12	41	32	1	0	1739459946832	f
12	16	1	-1	0	1739459948926	f
12	40	4	-1	0	1739459949037	f
12	43	7	-1	0	1739459950644	f
12	43	28	1	0	1739459954103	f
12	16	13	-1	0	1739459966400	f
12	17	19	-1	0	1739459968274	f
12	14	7	0	0	1739459968393	f
12	40	34	1	0	1739459969200	f
12	14	2	-1	0	1739459972487	f
12	40	17	-1	0	1739459982184	f
12	16	7	0	0	1739459983751	f
12	14	6	0	0	1739459984470	f
12	14	19	0	0	1739459989797	f
12	40	0	1	0	1739459993092	f
12	14	8	0	0	1739459999520	f
12	44	4	-1	0	1739460195941	f
12	8	1	-1	0	1739460230733	f
12	44	7	0	0	1739460252639	f
12	8	30	1	0	1739460257235	f
12	45	19	1	0	1739461077189	f
12	49	19	-1	0	1739461980281	f
12	52	3	-1	0	1739462818977	f
12	53	14	-1	0	1739462850016	f
12	53	0	1	0	1739462858503	f
12	52	4	-1	0	1739462863371	f
12	52	33	-1	0	1739462870650	f
12	5	14	-1	0	1739463242240	f
12	5	35	-1	0	1739463249614	f
12	12	35	-1	0	1739463865914	f
12	35	8	-1	0	1739464303888	f
12	11	21	-1	0	1739464920835	f
12	8	35	-1	0	1739465406519	f
12	8	21	-1	0	1739465435940	f
12	8	36	-1	0	1739465438445	f
12	8	25	-1	0	1739465465759	f
12	2	26	-1	0	1739465529874	f
2	28	20	-1	0	1736657866560	f
2	27	5	-1	0	1736657869336	f
2	28	27	-1	0	1736657875700	f
2	27	35	-1	0	1736657881564	f
2	27	3	-1	0	1736657887848	f
2	28	25	0	0	1736657889534	f
2	28	31	1	0	1736657901013	f
2	28	10	-1	0	1736657903120	f
2	28	34	1	0	1736657918241	f
2	27	34	1	0	1736657921769	f
2	75	21	-1	0	1736740490617	f
2	75	24	-1	0	1736740496708	f
2	75	61	-1	0	1736740499685	f
2	75	57	0	0	1736740513412	f
4	0	1	0	0	1736754959984	f
5	1	1	-1	0	1736794365780	f
5	6	14	0	0	1736795450871	f
5	13	13	-1	0	1736802790703	f
5	16	21	-1	0	1736807068410	f
5	16	17	-1	0	1736807085985	f
5	16	11	-1	0	1736807091248	f
5	18	7	-1	0	1736814134524	f
5	18	13	-1	0	1736814145518	f
2	78	53	-1	0	1736827468137	f
2	78	9	-1	0	1736827480759	f
2	11	58	1	0	1736879515632	f
2	82	30	1	0	1736880443638	f
2	82	67	-1	0	1736880789282	f
2	85	40	-1	0	1736881726293	f
2	85	41	-1	0	1736881732992	f
2	85	31	-1	0	1736881739322	f
2	85	43	-1	0	1736881755423	f
2	85	20	-1	0	1736881764244	f
2	85	15	-1	0	1736881783621	f
2	85	46	-1	0	1736881786829	f
2	85	72	0	0	1736881805410	f
2	85	60	-1	0	1736881815948	f
2	85	12	-1	0	1736881823890	f
2	85	45	-1	0	1736881832511	f
2	85	59	-1	0	1736881839492	f
2	85	52	1	0	1736881858482	f
2	85	37	0	0	1736881869800	f
2	85	42	-1	0	1736881879566	f
2	85	36	-1	0	1736881885131	f
2	87	2	-1	0	1736916090536	f
2	87	35	0	0	1736916534490	f
5	12	4	0	0	1736985941962	f
5	12	11	1	0	1736985953632	f
5	12	9	-1	0	1736985957448	f
5	12	8	1	0	1736985960490	f
5	12	6	-1	0	1736985969041	f
5	12	5	-1	0	1736985975682	f
2	88	35	-1	0	1736994930956	f
2	89	31	-1	0	1737002347948	f
2	89	34	-1	0	1737002366162	f
2	89	10	-1	0	1737002386694	f
2	90	50	0	0	1737041402493	f
2	90	55	1	0	1737041780794	f
2	90	29	-1	0	1737041802364	f
2	91	38	-1	0	1737265382774	f
3	7	3	-1	0	1737731280259	f
9	6	7	-1	0	1739381424062	f
12	0	26	-1	0	1739382325355	f
12	8	4	-1	0	1739454619252	f
12	9	4	-1	0	1739454624492	f
12	12	0	1	0	1739454663348	f
12	13	17	-1	0	1739454664841	f
12	15	0	-1	0	1739454699535	f
12	17	0	-1	0	1739454708596	f
12	5	19	-1	0	1739455143884	f
12	22	23	-1	0	1739455598359	f
12	30	28	-1	0	1739455626216	f
12	29	0	-1	0	1739455640996	f
12	4	6	-1	0	1739456037207	f
12	0	30	0	0	1739456061556	f
12	2	6	-1	0	1739456065448	f
9	11	32	-1	0	1739456075239	f
12	5	30	-1	0	1739456469600	f
12	18	23	-1	0	1739457536854	f
12	4	7	0	0	1739457545651	f
12	21	31	-1	0	1739458098933	f
12	12	32	-1	0	1739458104689	f
12	34	27	1	0	1739458671156	f
12	5	3	-1	0	1739458681527	f
12	36	28	1	0	1739459655480	f
12	35	29	0	0	1739459668189	f
12	40	19	1	0	1739459859046	f
12	41	19	-1	0	1739459863165	f
12	7	31	-1	0	1739459873932	f
12	39	33	-1	0	1739459875511	f
12	41	23	-1	0	1739459875690	f
12	27	33	-1	0	1739459877743	f
12	41	1	-1	0	1739459878473	f
12	42	3	-1	0	1739459894726	f
12	7	30	-1	0	1739459899793	f
12	16	33	-1	0	1739459904570	f
12	14	34	0	0	1739459905647	f
12	43	13	-1	0	1739459917920	f
12	14	29	0	0	1739459945673	f
12	17	27	0	0	1739459945803	f
12	17	6	-1	0	1739459950786	f
12	40	2	-1	0	1739459952139	f
12	14	27	-1	0	1739459952267	f
12	41	29	1	0	1739459959680	f
12	40	30	1	0	1739459962380	f
12	44	28	1	0	1739460200590	f
12	46	6	-1	0	1739461093160	f
12	51	27	0	0	1739462268004	f
12	51	11	1	0	1739462299360	f
12	51	2	-1	0	1739462302308	f
12	51	23	-1	0	1739462305779	f
12	51	17	-1	0	1739462320540	f
12	34	25	-1	0	1739462821748	f
12	18	35	-1	0	1739463330508	f
12	16	35	-1	0	1739463342801	f
12	18	14	-1	0	1739463342932	f
12	56	22	-1	0	1739463945527	f
12	56	2	-1	0	1739463963264	f
12	15	30	-1	0	1739464309884	f
12	2	21	-1	0	1739464928189	f
12	11	18	-1	0	1739464930971	f
12	8	22	1	0	1739465416761	f
12	0	39	0	0	1739466009680	f
12	59	37	-1	0	1739466034854	f
12	17	26	-1	0	1739466374031	f
12	5	41	-1	0	1739468239920	f
12	60	35	-1	0	1739596038329	f
2	28	30	1	0	1736657868385	f
2	28	37	-1	0	1736657884052	f
2	28	9	0	0	1736657891664	f
2	27	22	1	0	1736657892930	f
2	28	22	0	0	1736657898042	f
2	27	36	-1	0	1736657901941	f
2	28	33	1	0	1736657906289	f
2	27	13	1	0	1736657908722	f
2	28	35	1	0	1736657909043	f
2	28	29	-1	0	1736657911988	f
2	28	13	0	0	1736657914623	f
2	28	23	-1	0	1736657920594	f
2	28	32	1	0	1736657924985	f
2	29	1	1	0	1736657926210	f
2	28	15	0	0	1736657928724	f
2	27	33	1	0	1736657932688	f
2	28	24	1	0	1736657932850	f
2	28	17	-1	0	1736657934604	f
2	28	36	-1	0	1736657936439	f
2	29	34	-1	0	1736657937270	f
2	28	28	1	0	1736657938160	f
2	27	23	-1	0	1736657938943	f
2	28	21	-1	0	1736657944297	f
2	27	32	-1	0	1736657946595	f
2	28	12	1	0	1736657946596	f
2	29	39	-1	0	1736657946823	f
2	28	5	-1	0	1736657948345	f
2	29	6	-1	0	1736657949306	f
2	28	11	0	0	1736657950789	f
2	29	2	1	0	1736657965049	f
2	27	24	1	0	1736657967905	f
2	29	38	-1	0	1736657971530	f
2	27	28	-1	0	1736657974368	f
2	29	27	0	0	1736657987126	f
2	27	37	-1	0	1736657989119	f
2	29	35	-1	0	1736657992436	f
2	27	25	-1	0	1736657994401	f
2	27	20	-1	0	1736657999131	f
2	29	5	0	0	1736658001609	f
2	27	21	-1	0	1736658004110	f
2	27	12	0	0	1736658010729	f
2	27	15	-1	0	1736658013902	f
2	27	17	-1	0	1736658016782	f
2	29	36	-1	0	1736658018941	f
2	27	11	-1	0	1736658019841	f
2	29	20	0	0	1736658026587	f
2	27	29	-1	0	1736658029912	f
2	29	30	-1	0	1736658035857	f
2	29	33	-1	0	1736658046942	f
2	29	31	-1	0	1736658051786	f
2	29	37	-1	0	1736658065317	f
2	29	17	-1	0	1736658067963	f
2	29	32	-1	0	1736658084660	f
2	0	40	0	0	1736658087582	f
2	29	13	-1	0	1736658087793	f
2	29	25	-1	0	1736658093021	f
2	29	40	-1	0	1736658099800	f
2	29	9	-1	0	1736658102325	f
2	29	12	-1	0	1736658106570	f
2	29	23	0	0	1736658115527	f
2	29	29	-1	0	1736658125951	f
2	29	15	-1	0	1736658132097	f
2	29	28	-1	0	1736658137107	f
2	29	3	-1	0	1736658145531	f
2	29	11	-1	0	1736658148333	f
2	29	10	-1	0	1736658152626	f
2	29	21	-1	0	1736658159482	f
2	30	6	1	0	1736658160280	f
2	29	22	-1	0	1736658163349	f
2	30	2	1	0	1736658165664	f
2	0	41	0	0	1736658171671	f
2	29	24	1	0	1736658172197	f
2	30	34	-1	0	1736658174440	f
2	30	38	-1	0	1736658180293	f
2	30	17	-1	0	1736658183478	f
2	30	1	1	0	1736658185367	f
2	29	41	0	0	1736658190483	f
2	30	39	-1	0	1736658190596	f
2	30	35	-1	0	1736658193926	f
2	30	30	0	0	1736658197308	f
2	30	11	0	0	1736658199861	f
2	30	28	-1	0	1736658202891	f
2	30	40	-1	0	1736658208103	f
2	30	33	-1	0	1736658211047	f
2	30	36	-1	0	1736658213441	f
2	30	20	-1	0	1736658214808	f
2	30	31	-1	0	1736658218090	f
2	30	25	-1	0	1736658219991	f
2	30	12	-1	0	1736658222546	f
2	30	27	-1	0	1736658226070	f
2	30	21	-1	0	1736658228060	f
2	30	9	-1	0	1736658229930	f
2	30	13	-1	0	1736658232605	f
2	30	23	-1	0	1736658235715	f
2	30	32	-1	0	1736658240431	f
2	30	10	-1	0	1736658242213	f
2	30	3	-1	0	1736658244014	f
2	30	5	-1	0	1736658245424	f
2	30	15	-1	0	1736658247160	f
2	30	37	-1	0	1736658252131	f
2	30	29	-1	0	1736658256068	f
2	30	41	0	0	1736658264025	f
2	30	22	0	0	1736658267876	f
2	30	24	0	0	1736658275014	f
2	15	38	0	0	1736658725415	f
2	15	40	-1	0	1736658733478	f
2	0	42	-1	0	1736659014858	f
2	0	43	-1	0	1736659124753	f
2	0	44	-1	0	1736659147408	f
2	0	45	0	0	1736659968264	f
2	13	42	-1	0	1736659995878	f
2	13	39	-1	0	1736660000997	f
2	13	43	-1	0	1736660002921	f
2	13	44	-1	0	1736660004344	f
2	13	40	-1	0	1736660007582	f
2	13	38	-1	0	1736660009645	f
2	13	45	-1	0	1736660016097	f
2	13	41	-1	0	1736660019536	f
2	13	46	-1	0	1736660283934	f
2	0	47	0	0	1736661684719	f
2	31	6	1	0	1736663252764	f
2	31	43	-1	0	1736663266396	f
2	31	2	1	0	1736663274182	f
2	31	46	-1	0	1736663277125	f
2	31	40	-1	0	1736663280415	f
2	31	30	-1	0	1736663284153	f
2	31	13	-1	0	1736663293480	f
2	31	38	-1	0	1736663295339	f
2	31	1	1	0	1736663297379	f
2	31	39	-1	0	1736663309188	f
2	31	45	-1	0	1736663312871	f
2	31	36	-1	0	1736663316891	f
2	31	10	-1	0	1736663318828	f
2	31	34	-1	0	1736663327577	f
2	31	42	-1	0	1736663332912	f
2	31	23	-1	0	1736663340178	f
2	31	33	-1	0	1736663347128	f
2	31	35	-1	0	1736663356152	f
2	76	6	1	0	1736742667051	f
2	76	30	0	0	1736742683759	f
2	76	42	-1	0	1736742696841	f
2	76	66	-1	0	1736742711374	f
2	76	70	-1	0	1736742722354	f
2	76	43	-1	0	1736742726372	f
2	76	31	-1	0	1736742740725	f
2	76	21	-1	0	1736742750557	f
2	76	25	-1	0	1736742757752	f
2	76	54	-1	0	1736742780824	f
2	76	35	-1	0	1736742787827	f
2	76	48	0	0	1736742809892	f
4	0	2	0	0	1736755000132	f
5	1	10	1	0	1736794388778	f
5	1	8	-1	0	1736794398840	f
5	7	13	-1	0	1736795556253	f
5	7	5	-1	0	1736795562525	f
5	8	13	-1	0	1736795571282	f
5	13	15	-1	0	1736802795159	f
5	16	16	1	0	1736807072206	f
5	16	20	0	0	1736807077583	f
5	16	13	-1	0	1736807089048	f
5	16	3	-1	0	1736807090433	f
5	16	2	-1	0	1736807113703	f
5	16	12	1	0	1736807121903	f
5	18	5	0	0	1736814139572	f
5	18	19	-1	0	1736814160317	f
2	78	5	-1	0	1736827505047	f
2	78	71	0	0	1736827510297	f
2	11	62	0	0	1736879541867	f
2	82	1	0	0	1736880459009	f
2	82	59	1	0	1736880816031	f
2	85	13	-1	0	1736881753240	f
2	85	23	-1	0	1736881762294	f
2	85	33	-1	0	1736881775054	f
2	85	47	-1	0	1736881798512	f
2	85	48	0	0	1736881811662	f
2	85	58	-1	0	1736881827324	f
2	85	66	-1	0	1736881844383	f
2	87	6	-1	0	1736916093680	f
2	87	3	-1	0	1736916537644	f
5	12	3	-1	0	1736985944482	f
5	12	22	-1	0	1736985950470	f
5	12	21	-1	0	1736985951666	f
5	12	14	-1	0	1736985964966	f
5	12	18	-1	0	1736985970903	f
5	12	15	1	0	1736985974174	f
5	12	19	1	0	1736985976898	f
5	12	20	-1	0	1736985982519	f
2	88	46	-1	0	1736994937617	f
2	88	21	-1	0	1736994954722	f
2	89	63	-1	0	1737002368972	f
2	89	33	-1	0	1737002398998	f
2	89	48	1	0	1737002417962	f
2	90	44	0	0	1737041409827	f
2	90	68	1	0	1737041795750	f
2	90	66	0	0	1737041827509	f
2	90	3	-1	0	1737041834010	f
2	91	21	-1	0	1737265389964	f
2	91	43	-1	0	1737265392375	f
2	91	25	-1	0	1737265407432	f
2	91	45	-1	0	1737265425852	f
11	0	0	0	0	1739296894842	f
11	9	2	-1	0	1739302658326	f
9	0	0	-1	0	1739317205304	f
9	6	8	-1	0	1739381437641	f
12	0	27	-1	0	1739382331427	f
12	7	4	-1	0	1739454620418	f
12	2	17	-1	0	1739454624567	f
12	7	0	1	0	1739454627130	f
12	9	0	1	0	1739454631099	f
12	5	27	-1	0	1739455151008	f
12	29	19	-1	0	1739455603003	f
12	29	27	-1	0	1739455612598	f
12	29	4	-1	0	1739455630502	f
12	2	30	0	0	1739456081589	f
12	22	11	-1	0	1739456506995	f
12	11	7	1	0	1739457546314	f
12	12	31	-1	0	1739458099143	f
12	5	2	-1	0	1739458679741	f
12	30	3	-1	0	1739458688090	f
12	12	2	-1	0	1739458707983	f
12	0	34	0	0	1739458745061	f
12	36	31	-1	0	1739459661649	f
12	13	3	-1	0	1739459670851	f
12	35	33	-1	0	1739459678617	f
12	36	3	-1	0	1739459690777	f
12	37	2	-1	0	1739459707421	f
12	38	13	-1	0	1739459879364	f
12	17	23	-1	0	1739459880517	f
12	41	3	-1	0	1739459884131	f
12	39	32	-1	0	1739459885629	f
12	44	27	-1	0	1739460204144	f
12	45	32	0	0	1739461110840	f
12	51	31	-1	0	1739462274381	f
12	52	0	-1	0	1739462822428	f
12	36	25	-1	0	1739462825143	f
12	52	17	-1	0	1739462825592	f
12	52	13	-1	0	1739462828809	f
12	52	27	-1	0	1739462834097	f
12	52	14	-1	0	1739462841000	f
12	52	25	-1	0	1739462855027	f
12	10	14	-1	0	1739462875295	f
12	18	25	-1	0	1739463334962	f
12	56	17	0	0	1739463960186	f
12	15	28	1	0	1739464320692	f
12	34	18	-1	0	1739464997845	f
12	8	22	1	0	1739465416847	f
12	2	39	-1	0	1739466053426	f
12	59	36	-1	0	1739466068218	f
12	16	39	-1	0	1739466080998	f
12	59	7	-1	0	1739466085988	f
12	17	38	-1	0	1739466378699	f
12	17	39	-1	0	1739466383243	f
12	2	41	0	0	1739468305715	f
12	60	21	-1	0	1739596043322	f
12	60	37	0	0	1739596046772	f
2	31	44	0	0	1736663352921	f
2	31	47	-1	0	1736663365528	f
2	76	2	0	0	1736742672659	f
2	76	9	-1	0	1736742679776	f
2	76	29	0	0	1736742691698	f
2	76	45	-1	0	1736742699861	f
2	76	69	0	0	1736742706873	f
2	76	72	-1	0	1736742716267	f
2	76	34	-1	0	1736742736341	f
2	76	27	-1	0	1736742747769	f
2	76	3	-1	0	1736742755637	f
2	76	55	0	0	1736742768193	f
2	76	22	0	0	1736742774532	f
2	76	23	-1	0	1736742783527	f
2	76	5	0	0	1736742825118	f
2	76	40	-1	0	1736742833261	f
2	76	33	-1	0	1736742843420	f
2	76	57	0	0	1736742863164	f
2	76	56	0	0	1736742877178	f
2	76	65	-1	0	1736742890180	f
2	76	28	0	0	1736742916244	f
2	76	52	0	0	1736742944042	f
2	76	49	0	0	1736742951575	f
2	76	11	-1	0	1736742958747	f
4	0	3	0	0	1736755015553	f
5	1	5	-1	0	1736794394966	f
5	8	7	-1	0	1736795556753	f
5	7	16	0	0	1736795565854	f
5	7	4	-1	0	1736795572050	f
5	13	5	-1	0	1736802797614	f
5	13	7	-1	0	1736802814428	f
5	16	1	1	0	1736807074862	f
5	16	19	-1	0	1736807083325	f
5	16	10	-1	0	1736807087883	f
5	16	9	-1	0	1736807089749	f
5	16	7	-1	0	1736807102443	f
5	18	18	-1	0	1736814152406	f
5	18	17	-1	0	1736814171736	f
5	18	4	-1	0	1736814183767	f
2	78	55	0	0	1736827542965	f
2	78	61	0	0	1736827549226	f
2	11	57	0	0	1736879545624	f
2	82	40	-1	0	1736880464994	f
2	82	31	-1	0	1736880472551	f
2	82	28	-1	0	1736880826401	f
2	82	25	-1	0	1736880853621	f
2	85	44	-1	0	1736881852431	f
2	85	57	0	0	1736881873551	f
2	85	53	-1	0	1736881881864	f
2	87	21	-1	0	1736916102414	f
2	87	71	0	0	1736916607858	f
5	12	16	1	0	1736985946424	f
5	12	7	-1	0	1736985955665	f
5	12	2	-1	0	1736985958957	f
5	12	1	-1	0	1736985962003	f
5	12	17	1	0	1736985972598	f
5	12	13	-1	0	1736985980361	f
2	88	64	1	0	1736994974252	f
2	89	58	-1	0	1737002374228	f
2	89	27	1	0	1737002384514	f
2	89	17	-1	0	1737002389645	f
2	89	37	-1	0	1737002407768	f
2	89	40	-1	0	1737002442634	f
2	89	68	0	0	1737002448686	f
2	89	67	-1	0	1737002481947	f
2	90	53	1	0	1737041422753	f
2	90	56	1	0	1737041445693	f
2	90	65	1	0	1737041811201	f
2	91	9	-1	0	1737265431232	f
2	91	28	-1	0	1737265435080	f
2	91	22	-1	0	1737265437644	f
2	91	51	-1	0	1737265443132	f
2	91	20	0	0	1737265455453	f
2	91	32	-1	0	1737265463579	f
2	91	49	0	0	1737265471708	f
2	91	55	1	0	1737265509026	f
2	91	50	-1	0	1737265516045	f
2	91	36	-1	0	1737265527663	f
11	0	1	0	0	1739296903352	f
11	10	0	1	0	1739308139115	f
9	1	1	-1	0	1739317231804	f
9	6	9	-1	0	1739381449395	f
9	5	13	-1	0	1739383570925	f
9	5	15	-1	0	1739383607556	f
9	5	18	-1	0	1739383629466	f
9	5	19	-1	0	1739383635993	f
9	5	20	-1	0	1739383641350	f
9	5	22	-1	0	1739383655592	f
9	5	23	-1	0	1739383662554	f
9	5	26	-1	0	1739383680900	f
9	5	27	-1	0	1739383688690	f
12	9	17	-1	0	1739454641030	f
12	3	17	-1	0	1739454651907	f
12	5	23	-1	0	1739455154523	f
12	29	17	-1	0	1739455618556	f
12	30	27	1	0	1739455642615	f
12	11	30	1	0	1739456097500	f
12	32	30	1	0	1739456513592	f
12	18	30	1	0	1739457550869	f
12	18	11	0	0	1739457558832	f
12	3	23	-1	0	1739458143975	f
12	3	32	0	0	1739458151256	f
12	4	3	-1	0	1739458680383	f
12	4	2	-1	0	1739458682816	f
12	30	2	-1	0	1739458684158	f
12	36	13	0	0	1739459668992	f
12	41	30	1	0	1739459899087	f
12	39	29	1	0	1739459899689	f
12	8	6	1	0	1739460236459	f
12	8	32	1	0	1739460273114	f
12	44	8	0	0	1739460280482	f
12	45	27	-1	0	1739461138743	f
12	45	11	0	0	1739461168736	f
12	51	13	-1	0	1739462277704	f
12	51	33	-1	0	1739462284939	f
12	36	14	-1	0	1739462829974	f
12	11	33	-1	0	1739463347735	f
12	3	23	-1	0	1739463395497	f
12	3	35	-1	0	1739463405801	f
12	56	35	-1	0	1739463966618	f
12	56	3	-1	0	1739463969892	f
12	56	23	-1	0	1739463986022	f
12	29	8	1	0	1739464333298	f
12	34	37	0	0	1739465014703	f
12	3	23	-1	0	1739465061070	f
12	3	37	-1	0	1739465097267	f
12	27	36	-1	0	1739465099268	f
12	12	18	-1	0	1739465101220	f
12	8	37	1	0	1739465419919	f
12	8	14	-1	0	1739465443577	f
12	59	4	-1	0	1739466061663	f
12	10	26	-1	0	1739466463257	f
12	2	40	-1	0	1739468310600	f
12	60	42	-1	0	1739596117717	f
2	31	32	0	0	1736663379016	f
2	31	22	1	0	1736663382186	f
2	31	31	0	0	1736663399243	f
2	31	15	-1	0	1736663401836	f
2	31	3	-1	0	1736663404244	f
2	31	27	-1	0	1736663408192	f
2	31	9	-1	0	1736663410136	f
2	31	12	-1	0	1736663420329	f
2	31	28	-1	0	1736663423953	f
2	31	21	-1	0	1736663428402	f
2	31	5	-1	0	1736663432999	f
2	31	11	1	0	1736663441261	f
2	31	25	-1	0	1736663443948	f
2	31	41	1	0	1736663461088	f
2	31	17	-1	0	1736663463493	f
2	31	37	-1	0	1736663466957	f
2	31	29	1	0	1736663476557	f
2	31	24	-1	0	1736663480953	f
2	31	20	-1	0	1736663487786	f
2	31	48	-1	0	1736663636885	f
2	31	49	-1	0	1736663822231	f
2	31	50	-1	0	1736663834228	f
2	32	30	-1	0	1736664442727	f
2	32	40	-1	0	1736664446565	f
2	32	42	-1	0	1736664450392	f
2	32	2	-1	0	1736664453483	f
2	32	49	-1	0	1736664458125	f
2	32	45	-1	0	1736664464430	f
2	32	6	1	0	1736664468487	f
2	32	50	-1	0	1736664471957	f
2	32	39	-1	0	1736664477169	f
2	32	38	-1	0	1736664479309	f
2	32	9	-1	0	1736664481548	f
2	32	43	-1	0	1736664483832	f
2	32	1	1	0	1736664487442	f
2	32	35	-1	0	1736664489850	f
2	32	48	-1	0	1736664496265	f
2	32	3	-1	0	1736664498911	f
2	32	46	-1	0	1736664502771	f
2	32	34	-1	0	1736664506411	f
2	32	31	-1	0	1736664510110	f
2	32	24	0	0	1736664640488	f
2	32	47	0	0	1736664645147	f
2	32	10	-1	0	1736664647636	f
2	32	22	0	0	1736664651530	f
2	32	36	-1	0	1736664654970	f
2	32	20	0	0	1736664658596	f
2	32	33	-1	0	1736664661568	f
2	32	27	-1	0	1736664664736	f
2	32	11	0	0	1736664667481	f
2	32	28	-1	0	1736664671966	f
2	32	13	-1	0	1736664676805	f
2	32	29	-1	0	1736664681068	f
2	32	44	-1	0	1736664683353	f
2	32	32	0	0	1736664706715	f
2	32	12	-1	0	1736664709555	f
2	32	25	-1	0	1736664713406	f
2	32	41	0	0	1736664719245	f
2	32	15	-1	0	1736664721527	f
2	32	5	-1	0	1736664723389	f
2	32	17	-1	0	1736664725471	f
2	32	37	0	0	1736664730227	f
2	32	21	-1	0	1736664733198	f
2	32	23	0	0	1736664737456	f
2	33	46	-1	0	1736666020919	f
2	33	1	1	0	1736666023538	f
2	33	48	-1	0	1736666034383	f
2	33	30	-1	0	1736666039316	f
2	33	6	1	0	1736666041219	f
2	33	2	1	0	1736666043423	f
2	33	27	-1	0	1736666049648	f
2	33	42	-1	0	1736666055588	f
2	33	45	-1	0	1736666061337	f
2	33	49	1	0	1736666067664	f
2	33	31	-1	0	1736666072881	f
2	33	33	1	0	1736666081095	f
2	33	43	0	0	1736666084166	f
2	33	28	-1	0	1736666088830	f
2	33	47	-1	0	1736666096096	f
2	33	40	-1	0	1736666099490	f
2	33	9	-1	0	1736666102034	f
2	33	44	-1	0	1736666105515	f
2	33	13	-1	0	1736666109370	f
2	33	24	1	0	1736666114142	f
2	33	17	-1	0	1736666117638	f
2	33	39	-1	0	1736666123399	f
2	33	32	-1	0	1736666129845	f
2	33	34	-1	0	1736666134556	f
2	33	37	-1	0	1736666139335	f
2	33	3	-1	0	1736666142553	f
2	33	50	-1	0	1736666146534	f
2	33	38	-1	0	1736666148469	f
2	33	22	1	0	1736666153459	f
2	33	35	-1	0	1736666160901	f
2	33	21	-1	0	1736666165124	f
2	33	20	-1	0	1736666167472	f
2	33	10	-1	0	1736666171058	f
2	33	29	-1	0	1736666181014	f
2	33	25	-1	0	1736666185801	f
2	33	12	-1	0	1736666188159	f
2	33	36	-1	0	1736666191731	f
2	33	5	-1	0	1736666194458	f
2	33	23	-1	0	1736666197771	f
2	33	11	1	0	1736666201000	f
2	33	15	-1	0	1736666204062	f
2	33	41	-1	0	1736666211718	f
2	0	51	0	0	1736666547058	f
2	0	50	0	0	1736666757483	f
2	0	46	-1	0	1736666760591	f
2	0	48	0	0	1736666782162	f
2	0	49	0	0	1736666784315	f
2	0	52	0	0	1736666930825	f
2	34	53	-1	0	1736669336139	f
2	35	6	0	0	1736673036880	f
2	35	12	0	0	1736673042964	f
2	35	30	0	0	1736673047231	f
2	11	30	-1	0	1736676176237	f
2	11	39	-1	0	1736676181649	f
2	11	42	-1	0	1736676185688	f
2	11	49	1	0	1736676193058	f
2	11	43	-1	0	1736676195638	f
2	11	47	-1	0	1736676201920	f
2	11	45	-1	0	1736676210110	f
2	11	38	-1	0	1736676213568	f
2	11	37	-1	0	1736676226398	f
2	11	44	1	0	1736676231887	f
2	11	34	-1	0	1736676240378	f
2	11	48	1	0	1736676258597	f
2	11	31	-1	0	1736676262407	f
2	11	35	-1	0	1736676265798	f
2	11	54	-1	0	1736676325380	f
2	11	55	-1	0	1736676364798	f
2	11	50	-1	0	1736676695159	f
2	11	46	-1	0	1736676701509	f
2	11	40	-1	0	1736676705760	f
2	11	28	-1	0	1736676711239	f
2	11	33	-1	0	1736676724738	f
2	11	53	-1	0	1736676728508	f
2	11	36	-1	0	1736676737197	f
2	11	41	1	0	1736676767837	f
2	11	32	-1	0	1736676779137	f
2	11	52	-1	0	1736676783017	f
2	11	51	-1	0	1736676787287	f
2	11	29	-1	0	1736676792747	f
2	36	6	1	0	1736678082209	f
2	36	55	-1	0	1736678089339	f
2	36	46	-1	0	1736678100297	f
2	36	30	0	0	1736678106797	f
2	36	20	-1	0	1736678112727	f
2	36	42	-1	0	1736678117837	f
2	36	2	1	0	1736678124837	f
2	36	35	-1	0	1736678129880	f
2	36	48	-1	0	1736678137238	f
2	36	49	1	0	1736678151723	f
2	36	45	1	0	1736678163877	f
2	36	54	-1	0	1736678168932	f
2	36	1	1	0	1736678173087	f
2	36	53	-1	0	1736678176817	f
2	36	44	-1	0	1736678181958	f
2	36	47	-1	0	1736678192537	f
2	36	52	0	0	1736678199137	f
2	36	21	1	0	1736678205481	f
2	36	34	-1	0	1736678212537	f
2	36	50	0	0	1736678222057	f
2	36	38	1	0	1736678226017	f
2	36	36	-1	0	1736678231958	f
2	36	15	-1	0	1736678236767	f
2	36	39	1	0	1736678241938	f
2	36	22	0	0	1736678250052	f
2	36	13	-1	0	1736678255078	f
2	36	40	-1	0	1736678257897	f
2	36	31	-1	0	1736678264672	f
2	36	51	-1	0	1736678267462	f
2	36	33	-1	0	1736678273318	f
2	36	43	-1	0	1736678276007	f
2	36	5	-1	0	1736678279237	f
2	36	24	1	0	1736678287077	f
2	36	29	0	0	1736678295922	f
2	36	17	-1	0	1736678300117	f
2	36	27	-1	0	1736678308362	f
2	36	25	-1	0	1736678312819	f
2	36	12	-1	0	1736678317497	f
2	36	32	-1	0	1736678321397	f
2	36	9	-1	0	1736678329498	f
2	36	23	-1	0	1736678334862	f
2	36	37	-1	0	1736678341618	f
2	36	41	-1	0	1736678350532	f
2	36	3	-1	0	1736678354332	f
2	36	11	0	0	1736678360127	f
2	36	10	-1	0	1736678365702	f
2	36	28	-1	0	1736678370069	f
2	37	56	-1	0	1736679614051	f
2	38	32	-1	0	1736690333427	f
2	38	53	-1	0	1736690337777	f
2	38	30	0	0	1736690344946	f
2	38	6	1	0	1736690349590	f
2	38	34	-1	0	1736690357122	f
2	38	55	-1	0	1736690362237	f
2	38	48	0	0	1736690372673	f
2	38	2	0	0	1736690380643	f
2	38	54	-1	0	1736690384606	f
2	38	49	-1	0	1736690396367	f
2	38	43	-1	0	1736690399471	f
2	38	39	-1	0	1736690533210	f
2	38	1	1	0	1736690536680	f
2	38	42	-1	0	1736690542608	f
2	38	33	-1	0	1736690550013	f
2	39	30	1	0	1736696208703	f
2	39	55	-1	0	1736696215175	f
2	39	6	-1	0	1736696219543	f
2	39	10	-1	0	1736696222413	f
2	39	50	0	0	1736696227733	f
2	39	1	1	0	1736696229892	f
2	39	42	-1	0	1736696233312	f
2	39	2	0	0	1736696237268	f
2	39	49	0	0	1736696241687	f
2	39	38	-1	0	1736696247275	f
2	39	36	-1	0	1736696251148	f
2	39	44	-1	0	1736696253668	f
2	39	37	0	0	1736696258350	f
2	39	28	-1	0	1736696262689	f
2	11	56	-1	0	1736699155169	f
2	40	39	-1	0	1736699354428	f
2	13	55	-1	0	1736699358544	f
2	13	54	-1	0	1736699360193	f
2	13	53	-1	0	1736699362484	f
2	13	51	-1	0	1736699364764	f
2	13	52	-1	0	1736699369007	f
2	13	56	-1	0	1736699388017	f
2	13	49	0	0	1736699396493	f
2	13	48	0	0	1736699409535	f
2	13	50	0	0	1736699411420	f
2	13	47	-1	0	1736699429213	f
2	41	2	0	0	1736699599450	f
2	41	39	-1	0	1736699603864	f
2	41	55	-1	0	1736699607980	f
2	41	53	0	0	1736699614971	f
2	41	1	0	0	1736699626004	f
2	41	43	0	0	1736699630413	f
2	41	51	0	0	1736699636916	f
2	41	30	0	0	1736699641333	f
2	41	6	1	0	1736699649109	f
2	41	21	-1	0	1736699653945	f
2	41	40	-1	0	1736699659296	f
2	41	27	-1	0	1736699669496	f
2	41	56	-1	0	1736699678777	f
2	41	54	-1	0	1736699701623	f
2	41	44	-1	0	1736699714220	f
2	41	42	-1	0	1736699719063	f
2	41	34	-1	0	1736699726467	f
2	42	48	-1	0	1736699755072	f
2	42	6	0	0	1736699758063	f
2	42	30	1	0	1736699776467	f
2	42	2	0	0	1736699782193	f
2	42	1	0	0	1736699783500	f
2	43	54	-1	0	1736699797667	f
2	12	55	0	0	1736699805884	f
2	43	6	1	0	1736699806015	f
2	15	46	-1	0	1736699813840	f
2	12	45	-1	0	1736699815764	f
2	15	44	-1	0	1736699816356	f
2	43	1	1	0	1736699818330	f
2	43	35	-1	0	1736699825314	f
2	15	56	-1	0	1736699826258	f
2	43	33	-1	0	1736699830494	f
2	15	55	-1	0	1736699833393	f
2	15	53	-1	0	1736699836054	f
2	43	39	-1	0	1736699837015	f
2	15	54	-1	0	1736699841289	f
2	43	2	1	0	1736699841545	f
2	43	52	-1	0	1736699846338	f
2	15	52	-1	0	1736699847514	f
2	43	30	-1	0	1736699854545	f
2	15	45	0	0	1736699890865	f
2	43	15	-1	0	1736699895783	f
2	43	55	-1	0	1736699904653	f
2	76	1	1	0	1736742675908	f
2	76	67	-1	0	1736742718231	f
2	76	20	-1	0	1736742724729	f
2	76	71	-1	0	1736742730972	f
2	76	13	-1	0	1736742753444	f
2	76	44	-1	0	1736742759951	f
2	76	51	0	0	1736742777747	f
2	76	39	-1	0	1736742797095	f
4	0	4	0	0	1736755039857	f
5	1	3	-1	0	1736794401315	f
5	2	12	0	0	1736794422839	f
5	3	11	1	0	1736794459164	f
5	3	6	0	0	1736794492276	f
5	3	4	-1	0	1736794506867	f
5	3	1	1	0	1736794511569	f
5	3	2	1	0	1736794526338	f
5	8	1	0	0	1736795561257	f
5	7	1	0	0	1736795564893	f
5	8	12	0	0	1736795582689	f
5	9	13	-1	0	1736795587135	f
5	8	10	0	0	1736795592702	f
5	7	8	-1	0	1736795594442	f
5	8	14	-1	0	1736795605412	f
5	7	7	-1	0	1736795608794	f
5	8	16	0	0	1736795611542	f
5	7	6	0	0	1736795619869	f
5	7	12	0	0	1736795621508	f
5	7	10	-1	0	1736795629469	f
5	9	7	-1	0	1736795725789	f
5	9	5	0	0	1736795740493	f
5	9	15	-1	0	1736795745529	f
5	8	17	-1	0	1736795787959	f
5	13	4	-1	0	1736802801726	f
5	13	18	-1	0	1736802824139	f
5	13	20	-1	0	1736802845797	f
5	16	6	1	0	1736807108789	f
5	16	5	1	0	1736807138379	f
5	18	8	-1	0	1736814176014	f
2	78	63	-1	0	1736827548031	f
2	78	42	-1	0	1736827561069	f
2	79	2	0	0	1736879781103	f
2	83	2	0	0	1736880472180	f
2	83	1	0	0	1736880476447	f
2	82	69	1	0	1736880481752	f
2	82	70	1	0	1736880510791	f
2	82	36	-1	0	1736880830578	f
2	85	5	1	0	1736881888981	f
2	85	11	-1	0	1736881902982	f
2	85	62	0	0	1736881923472	f
2	85	49	-1	0	1736881926280	f
2	87	68	0	0	1736916124243	f
2	87	23	0	0	1736916147278	f
2	87	22	0	0	1736916615920	f
2	88	1	1	0	1736993941583	f
2	88	52	-1	0	1736993956131	f
2	88	40	-1	0	1736994979687	f
2	89	28	-1	0	1737002425585	f
2	89	54	-1	0	1737002431557	f
2	89	5	1	0	1737002455314	f
2	90	5	-1	0	1737041433896	f
2	90	9	-1	0	1737041837069	f
2	91	64	0	0	1737265494407	f
2	91	71	1	0	1737265501168	f
2	91	11	1	0	1737265530502	f
2	91	57	-1	0	1737265536034	f
11	1	1	-1	0	1739296921442	f
11	10	2	-1	0	1739308140927	f
11	13	10	-1	0	1739318781200	f
9	6	10	-1	0	1739381465646	f
9	5	14	-1	0	1739383587033	f
9	5	16	-1	0	1739383617736	f
9	5	17	-1	0	1739383623337	f
9	5	24	-1	0	1739383668334	f
12	10	17	-1	0	1739454654385	f
12	11	4	-1	0	1739454656462	f
12	11	17	-1	0	1739454662186	f
12	12	4	-1	0	1739454668413	f
12	17	17	-1	0	1739454705313	f
12	0	28	0	0	1739455245015	f
12	0	29	0	0	1739455658454	f
12	30	17	-1	0	1739455667976	f
12	31	23	-1	0	1739456152940	f
12	31	29	1	0	1739456168985	f
12	9	29	1	0	1739456217355	f
12	26	28	0	0	1739456219025	f
12	26	19	-1	0	1739456226734	f
12	26	11	0	0	1739456241946	f
12	26	23	-1	0	1739456248684	f
12	26	27	-1	0	1739456252582	f
12	22	8	0	0	1739456516825	f
12	32	4	0	0	1739456527427	f
12	32	28	1	0	1739456532936	f
12	18	6	-1	0	1739457562563	f
12	22	32	-1	0	1739458149091	f
12	30	33	-1	0	1739458720958	f
12	30	32	-1	0	1739458737198	f
12	34	34	0	0	1739458836891	f
12	36	33	-1	0	1739459674187	f
12	36	7	0	0	1739459687947	f
12	13	31	0	0	1739459694176	f
12	14	31	-1	0	1739459900246	f
12	30	1	-1	0	1739459901329	f
12	40	23	-1	0	1739459904102	f
12	35	34	-1	0	1739459904299	f
12	8	13	-1	0	1739460240074	f
12	8	34	-1	0	1739460245093	f
12	8	2	-1	0	1739460246808	f
12	8	33	-1	0	1739460260559	f
12	46	31	-1	0	1739461177554	f
12	51	3	-1	0	1739462280886	f
12	51	0	-1	0	1739462291989	f
12	53	27	0	0	1739462866614	f
12	10	25	-1	0	1739462878871	f
12	53	25	-1	0	1739462889292	f
12	55	1	-1	0	1739463386580	f
12	17	25	-1	0	1739463395815	f
12	55	35	-1	0	1739463398332	f
12	3	25	-1	0	1739463408579	f
12	56	30	0	0	1739464033007	f
12	57	11	1	0	1739464340721	f
12	56	8	1	0	1739464353243	f
12	35	37	-1	0	1739465059282	f
12	35	21	1	0	1739465093929	f
12	12	21	0	0	1739465095634	f
12	8	20	-1	0	1739465452037	f
12	59	25	-1	0	1739466064531	f
12	2	38	-1	0	1739466072945	f
12	10	38	-1	0	1739466471728	f
12	3	41	-1	0	1739468503109	f
12	60	38	1	0	1739596121907	f
2	43	46	-1	0	1736699858950	f
2	43	42	-1	0	1736699873522	f
2	76	38	0	0	1736742819178	f
2	76	63	0	0	1736742831321	f
2	76	10	-1	0	1736742835084	f
2	76	58	0	0	1736742854040	f
2	76	47	0	0	1736742871094	f
2	76	53	1	0	1736742887183	f
2	76	37	-1	0	1736742905573	f
2	76	46	-1	0	1736742925279	f
2	76	32	0	0	1736742936427	f
4	1	1	-1	0	1736756067769	f
5	1	2	1	0	1736794403226	f
5	8	15	-1	0	1736795577902	f
5	7	14	0	0	1736795584653	f
5	8	8	-1	0	1736795590490	f
5	8	11	0	0	1736795607731	f
5	8	9	0	0	1736795610501	f
5	8	4	-1	0	1736795620708	f
5	7	11	0	0	1736795622279	f
5	7	9	0	0	1736795627476	f
5	9	1	0	0	1736795730311	f
5	9	4	-1	0	1736795752172	f
5	9	18	-1	0	1736795934394	f
5	13	12	0	0	1736802835124	f
5	13	19	-1	0	1736802882816	f
5	16	14	-1	0	1736807140195	f
5	18	15	0	0	1736814202559	f
2	78	15	-1	0	1736827588480	f
2	79	1	1	0	1736879783898	f
2	79	63	-1	0	1736879805363	f
2	83	6	0	0	1736880473928	f
2	82	65	0	0	1736880500110	f
2	82	54	0	0	1736880837845	f
2	85	55	-1	0	1736881894154	f
2	85	61	0	0	1736881898661	f
2	85	32	0	0	1736881913551	f
2	87	17	-1	0	1736916127003	f
2	87	46	-1	0	1736916131997	f
2	87	66	-1	0	1736916632372	f
2	88	6	1	0	1736993944103	f
2	88	10	-1	0	1736993959471	f
2	88	65	1	0	1736994997897	f
2	89	69	-1	0	1737002435771	f
2	89	11	-1	0	1737002439507	f
2	89	66	-1	0	1737002466244	f
2	90	27	-1	0	1737041456810	f
2	90	41	-1	0	1737041847444	f
2	91	52	-1	0	1737265533170	f
11	1	0	-1	0	1739296923122	f
11	4	1	-1	0	1739296950850	f
11	6	0	0	0	1739296969754	f
11	10	3	-1	0	1739308174193	f
9	2	1	-1	0	1739345825869	f
9	6	11	-1	0	1739381475543	f
9	5	21	-1	0	1739383648005	f
9	5	25	-1	0	1739383674008	f
9	5	28	-1	0	1739383694906	f
12	13	4	0	0	1739454671941	f
12	16	17	-1	0	1739454692408	f
12	13	0	1	0	1739454701523	f
12	18	0	-1	0	1739454704520	f
12	16	4	0	0	1739454709851	f
12	21	4	-1	0	1739454727361	f
12	20	4	-1	0	1739454730512	f
12	4	28	-1	0	1739455293848	f
12	3	27	-1	0	1739455304555	f
12	3	28	0	0	1739455310256	f
12	11	29	1	0	1739455676721	f
12	31	6	1	0	1739456158008	f
12	32	19	1	0	1739456539478	f
12	32	8	1	0	1739456562011	f
12	18	29	0	0	1739457589370	f
12	10	6	-1	0	1739457603003	f
12	10	30	1	0	1739457621995	f
12	18	8	0	0	1739457633141	f
12	9	32	1	0	1739458151281	f
12	3	7	0	0	1739458163328	f
12	4	34	0	0	1739458790137	f
12	36	1	-1	0	1739459695157	f
12	37	1	-1	0	1739459703366	f
12	36	30	1	0	1739459705517	f
12	13	30	-1	0	1739459708219	f
12	7	27	-1	0	1739459910233	f
12	30	13	-1	0	1739459911798	f
12	43	34	-1	0	1739459913119	f
12	43	33	-1	0	1739459915812	f
12	14	23	-1	0	1739459921696	f
12	43	3	-1	0	1739459926002	f
12	17	34	0	0	1739459927256	f
12	43	17	-1	0	1739459928457	f
12	17	28	0	0	1739459955201	f
12	43	0	-1	0	1739459962382	f
12	41	4	-1	0	1739459966338	f
12	43	27	1	0	1739459976462	f
12	40	7	1	0	1739460019770	f
12	16	8	0	0	1739460030723	f
12	40	11	1	0	1739460032935	f
12	40	28	1	0	1739460039485	f
12	27	3	-1	0	1739460058403	f
12	8	3	-1	0	1739460264775	f
12	44	0	1	0	1739460264830	f
12	45	28	-1	0	1739461211168	f
12	51	1	-1	0	1739462311632	f
12	52	2	-1	0	1739462872802	f
12	53	1	-1	0	1739462876085	f
12	17	22	-1	0	1739463402218	f
12	3	14	-1	0	1739463412827	f
12	17	35	-1	0	1739463416494	f
12	17	14	-1	0	1739463435159	f
12	11	14	1	0	1739463477036	f
12	11	35	-1	0	1739463495377	f
12	11	3	-1	0	1739463500913	f
12	11	25	-1	0	1739463506986	f
12	56	0	1	0	1739464040929	f
12	56	6	-1	0	1739464045264	f
12	57	31	1	0	1739464351857	f
12	27	18	-1	0	1739465061195	f
12	27	25	-1	0	1739465063934	f
12	27	37	-1	0	1739465091117	f
12	3	18	-1	0	1739465091378	f
12	20	21	1	0	1739465091920	f
12	27	20	-1	0	1739465095040	f
12	12	37	-1	0	1739465105941	f
12	20	37	-1	0	1739465107065	f
12	53	30	1	0	1739465142047	f
12	53	37	-1	0	1739465177973	f
12	3	20	-1	0	1739465240321	f
12	21	18	-1	0	1739465535007	f
12	18	21	-1	0	1739465567014	f
12	59	39	-1	0	1739466147459	f
12	10	21	-1	0	1739466503036	f
12	18	26	-1	0	1739480171290	f
12	18	41	-1	0	1739480191932	f
12	60	23	-1	0	1739596125340	f
2	43	56	0	0	1736699868351	f
2	43	47	-1	0	1736699881374	f
2	43	50	-1	0	1736699886121	f
2	43	10	-1	0	1736699889605	f
2	43	53	-1	0	1736699892193	f
2	15	47	0	0	1736699899070	f
2	43	34	-1	0	1736699901631	f
2	43	12	-1	0	1736699908231	f
2	43	38	-1	0	1736699910522	f
2	15	51	1	0	1736699912402	f
2	43	43	-1	0	1736699913062	f
2	15	49	0	0	1736699916750	f
2	15	43	-1	0	1736699918377	f
2	43	31	-1	0	1736699919235	f
2	43	49	-1	0	1736699922417	f
2	15	50	-1	0	1736699928826	f
2	43	51	-1	0	1736699929064	f
2	43	40	-1	0	1736699932600	f
2	15	42	-1	0	1736699934426	f
2	43	11	-1	0	1736699935702	f
2	43	37	-1	0	1736699943571	f
2	15	48	1	0	1736699946732	f
2	43	21	-1	0	1736699947512	f
2	43	44	-1	0	1736699950017	f
2	43	13	-1	0	1736699953745	f
2	43	48	1	0	1736699968653	f
2	15	41	0	0	1736699969975	f
2	12	32	0	0	1736699970866	f
2	43	3	-1	0	1736699973349	f
2	43	28	-1	0	1736699978967	f
2	12	51	0	0	1736699980424	f
2	43	9	-1	0	1736699980971	f
2	44	51	0	0	1736699984898	f
2	12	42	-1	0	1736699987389	f
2	43	32	0	0	1736699988209	f
2	44	1	1	0	1736699988923	f
2	12	52	-1	0	1736699994435	f
2	43	25	-1	0	1736699995377	f
2	43	45	0	0	1736700003046	f
2	44	30	-1	0	1736700004300	f
2	43	20	-1	0	1736700006697	f
2	44	9	-1	0	1736700008035	f
2	43	23	-1	0	1736700010894	f
2	44	54	-1	0	1736700015845	f
2	12	54	-1	0	1736700016113	f
2	44	6	1	0	1736700019083	f
2	43	41	0	0	1736700020204	f
2	12	40	-1	0	1736700022274	f
2	43	5	-1	0	1736700024063	f
2	44	39	-1	0	1736700027823	f
2	43	27	-1	0	1736700028279	f
2	44	32	-1	0	1736700031997	f
2	43	22	-1	0	1736700034627	f
2	44	56	-1	0	1736700040713	f
2	43	36	-1	0	1736700041563	f
2	44	2	1	0	1736700044045	f
2	43	24	-1	0	1736700046294	f
2	43	17	-1	0	1736700048482	f
2	44	38	-1	0	1736700052033	f
2	43	29	-1	0	1736700054948	f
2	44	49	-1	0	1736700056183	f
2	44	21	-1	0	1736700060141	f
2	12	39	0	0	1736700090805	f
2	12	35	-1	0	1736700098204	f
2	44	55	-1	0	1736700101758	f
2	12	46	-1	0	1736700102306	f
2	44	5	-1	0	1736700105476	f
2	44	13	-1	0	1736700112276	f
2	12	33	-1	0	1736700114498	f
2	12	38	-1	0	1736700118611	f
2	44	48	0	0	1736700120815	f
2	12	56	0	0	1736700131454	f
2	12	44	-1	0	1736700138801	f
2	44	53	0	0	1736700138846	f
2	44	29	-1	0	1736700150956	f
2	44	12	-1	0	1736700154245	f
2	12	34	-1	0	1736700155227	f
2	44	42	-1	0	1736700158086	f
2	44	31	-1	0	1736700162916	f
2	44	52	-1	0	1736700165985	f
2	44	40	-1	0	1736700169705	f
2	44	28	-1	0	1736700175588	f
2	44	41	0	0	1736700201919	f
2	44	44	0	0	1736700206167	f
2	44	10	0	0	1736700206466	f
2	45	33	-1	0	1736700557289	f
2	45	30	-1	0	1736700569636	f
2	45	9	-1	0	1736700574016	f
2	45	43	-1	0	1736700702931	f
2	45	1	1	0	1736700706495	f
2	45	2	1	0	1736700712655	f
2	45	6	1	0	1736700715515	f
2	45	55	-1	0	1736700730835	f
2	45	46	-1	0	1736700740896	f
2	45	56	-1	0	1736700755957	f
2	45	42	-1	0	1736700761236	f
2	45	34	-1	0	1736700776616	f
2	45	15	-1	0	1736700779800	f
2	45	17	-1	0	1736700782876	f
2	45	38	-1	0	1736700786037	f
2	45	40	-1	0	1736700789996	f
2	45	36	0	0	1736700799191	f
2	45	39	-1	0	1736700812035	f
2	45	3	-1	0	1736700816998	f
2	45	13	-1	0	1736700820216	f
2	45	50	-1	0	1736700825496	f
2	45	54	-1	0	1736700831995	f
2	45	37	-1	0	1736700854342	f
2	45	44	0	0	1736700859657	f
2	46	2	-1	0	1736700869757	f
2	45	21	-1	0	1736700871476	f
2	45	52	0	0	1736700875557	f
2	45	22	0	0	1736700887535	f
2	45	45	0	0	1736700902856	f
2	45	25	-1	0	1736700908095	f
2	45	53	-1	0	1736700912145	f
2	45	47	-1	0	1736700938420	f
2	45	31	-1	0	1736700943781	f
2	45	51	0	0	1736700966097	f
2	45	41	0	0	1736700984236	f
2	45	27	0	0	1736700990755	f
2	45	35	-1	0	1736700994516	f
2	45	23	-1	0	1736701002641	f
2	45	11	-1	0	1736701007595	f
2	45	48	0	0	1736701021815	f
2	45	20	-1	0	1736701030176	f
2	45	49	-1	0	1736701066376	f
2	45	5	-1	0	1736701074036	f
2	45	12	-1	0	1736701079576	f
2	45	32	-1	0	1736701100280	f
2	45	28	-1	0	1736701107517	f
2	45	29	-1	0	1736701117097	f
2	45	24	0	0	1736701123356	f
2	45	10	-1	0	1736701128700	f
2	47	3	-1	0	1736701402309	f
2	47	2	-1	0	1736701417158	f
2	47	1	1	0	1736701420866	f
2	76	36	-1	0	1736742857933	f
2	76	12	-1	0	1736742859792	f
2	76	15	-1	0	1736742865200	f
2	76	68	-1	0	1736742897838	f
2	76	64	-1	0	1736742908001	f
4	0	5	0	0	1736756088846	f
4	0	6	0	0	1736756098054	f
4	1	6	-1	0	1736756110718	f
5	4	13	-1	0	1736794490397	f
5	3	7	-1	0	1736794501992	f
5	3	5	0	0	1736794521629	f
5	3	12	1	0	1736794542524	f
5	8	3	0	0	1736795580541	f
5	7	2	0	0	1736795585785	f
5	7	15	0	0	1736795589435	f
5	9	11	0	0	1736795595556	f
5	8	2	0	0	1736795609135	f
5	7	3	0	0	1736795612694	f
5	8	5	-1	0	1736795624561	f
5	13	8	-1	0	1736802841816	f
5	13	3	0	0	1736802848359	f
5	16	4	-1	0	1736807147744	f
5	19	16	0	0	1736814214374	f
2	78	52	0	0	1736827593393	f
2	79	6	1	0	1736879785514	f
2	82	45	-1	0	1736880520612	f
2	82	9	-1	0	1736880858289	f
2	82	5	-1	0	1736880861329	f
2	85	24	-1	0	1736881916868	f
2	85	50	-1	0	1736881928820	f
2	87	63	0	0	1736916158887	f
2	87	11	0	0	1736916639976	f
2	87	64	-1	0	1736916660383	f
2	88	2	-1	0	1736993950800	f
2	88	51	0	0	1736995006631	f
2	88	66	0	0	1736995021567	f
2	89	50	-1	0	1737002472212	f
2	90	57	0	0	1737041484842	f
2	90	22	1	0	1737041857958	f
2	34	5	-1	0	1737406068643	f
11	2	0	1	0	1739296933290	f
11	3	1	1	0	1739296943378	f
11	4	0	1	0	1739296950363	f
11	5	0	0	0	1739296960293	f
11	10	4	-1	0	1739308313962	f
9	2	0	-1	0	1739345831839	f
9	6	12	-1	0	1739381490697	f
9	5	8	0	0	1739384339208	f
12	12	17	-1	0	1739454672253	f
12	20	17	-1	0	1739454718151	f
12	23	4	-1	0	1739454734196	f
12	24	17	-1	0	1739454734756	f
12	20	0	1	0	1739454738074	f
12	24	0	0	0	1739454762129	f
12	22	0	1	0	1739454787319	f
12	25	4	-1	0	1739454790955	f
12	14	4	-1	0	1739454827844	f
12	3	23	-1	0	1739455295361	f
12	4	27	-1	0	1739455315167	f
12	30	29	-1	0	1739455683232	f
12	31	17	-1	0	1739456163797	f
12	32	27	1	0	1739456568381	f
12	18	28	0	0	1739457618018	f
12	10	7	1	0	1739457631666	f
12	11	32	1	0	1739458152234	f
12	9	31	-1	0	1739458159061	f
12	22	7	0	0	1739458159201	f
12	34	4	-1	0	1739458336115	f
12	2	34	0	0	1739458796734	f
12	36	29	1	0	1739459718300	f
12	35	32	0	0	1739459720410	f
12	36	4	0	0	1739459724379	f
12	13	32	-1	0	1739459729156	f
12	36	19	-1	0	1739459752417	f
12	13	33	0	0	1739459754642	f
12	13	28	1	0	1739459772236	f
12	7	8	1	0	1739459916530	f
12	14	1	-1	0	1739459917088	f
12	43	1	-1	0	1739459923783	f
12	14	28	1	0	1739459933883	f
12	17	7	0	0	1739459941718	f
12	16	30	1	0	1739459943602	f
12	8	7	-1	0	1739460277900	f
12	8	11	1	0	1739460281238	f
12	45	8	1	0	1739461238939	f
12	51	19	-1	0	1739462315231	f
12	51	28	0	0	1739462331385	f
12	54	25	-1	0	1739462896462	f
12	54	14	-1	0	1739462903941	f
12	54	27	0	0	1739462921599	f
12	31	1	1	0	1739462941133	f
12	54	4	-1	0	1739462953888	f
12	31	34	1	0	1739462963376	f
12	31	3	1	0	1739462966498	f
12	31	11	1	0	1739462979645	f
12	54	23	-1	0	1739463012067	f
12	54	34	-1	0	1739463039687	f
12	54	28	0	0	1739463045110	f
12	3	22	-1	0	1739463402388	f
12	0	36	0	0	1739464053287	f
12	56	14	-1	0	1739464067852	f
12	11	20	-1	0	1739464378028	f
12	11	36	1	0	1739464385208	f
12	34	36	-1	0	1739464429345	f
12	27	22	-1	0	1739465068838	f
12	30	18	-1	0	1739465083732	f
12	21	26	-1	0	1739465540410	f
12	5	39	-1	0	1739466161333	f
12	11	39	-1	0	1739466513845	f
12	11	38	-1	0	1739466517605	f
12	10	39	-1	0	1739466535203	f
12	10	37	-1	0	1739466537759	f
12	10	18	-1	0	1739466548849	f
12	18	40	-1	0	1739480177392	f
12	18	38	-1	0	1739480200593	f
12	60	43	-1	0	1739596156241	f
2	47	39	-1	0	1736701430182	f
2	47	33	-1	0	1736701434276	f
2	47	40	-1	0	1736701443759	f
2	47	52	-1	0	1736701459946	f
2	47	43	-1	0	1736701462379	f
2	76	59	-1	0	1736742910266	f
2	76	17	-1	0	1736742922963	f
2	76	61	-1	0	1736742927157	f
2	76	60	-1	0	1736742940395	f
2	76	62	0	0	1736742948462	f
2	76	41	0	0	1736742956561	f
4	1	0	1	0	1736756098359	f
4	1	3	-1	0	1736756104298	f
5	3	3	1	0	1736794510052	f
5	3	10	-1	0	1736794523645	f
5	3	13	1	0	1736794530716	f
5	3	9	-1	0	1736794533565	f
5	3	8	-1	0	1736794541037	f
5	8	6	-1	0	1736795633267	f
5	9	14	-1	0	1736795692908	f
5	9	6	-1	0	1736795718496	f
5	9	3	0	0	1736795728307	f
5	9	9	0	0	1736795963219	f
5	13	17	-1	0	1736802887269	f
5	13	14	-1	0	1736802903737	f
5	16	22	-1	0	1736807278401	f
5	20	22	0	0	1736814454272	f
5	20	10	0	0	1736814462281	f
5	20	2	0	0	1736814472664	f
5	20	13	1	0	1736814517846	f
5	20	6	-1	0	1736814525626	f
5	20	4	-1	0	1736814566206	f
5	20	15	0	0	1736814617797	f
5	20	14	-1	0	1736814656693	f
2	78	70	1	0	1736827597099	f
2	79	37	-1	0	1736879795696	f
2	80	6	1	0	1736879804356	f
2	82	27	-1	0	1736880527211	f
2	82	72	-1	0	1736880531750	f
2	82	37	-1	0	1736880865771	f
2	86	30	-1	0	1736882248310	f
2	87	60	0	0	1736916170661	f
2	87	54	-1	0	1736916645354	f
2	88	23	-1	0	1736994585304	f
2	88	17	-1	0	1736995023990	f
2	89	61	1	0	1737002530372	f
2	89	53	-1	0	1737002541788	f
2	90	31	-1	0	1737041490353	f
2	90	47	1	0	1737041883879	f
2	92	1	1	0	1737467377061	f
2	92	2	1	0	1737467383924	f
2	92	53	-1	0	1737467388462	f
2	92	37	-1	0	1737467393322	f
11	2	1	1	0	1739296934170	f
11	3	0	-1	0	1739296942915	f
11	5	1	1	0	1739296959223	f
11	6	1	0	0	1739296970122	f
11	11	4	-1	0	1739308727083	f
11	11	11	-1	0	1739351006932	f
12	0	0	-1	0	1739381900528	f
9	7	1	0	0	1739384417629	f
12	10	4	-1	0	1739454677409	f
12	10	0	1	0	1739454683348	f
9	8	31	-1	0	1739454685545	f
12	15	4	-1	0	1739454690576	f
12	19	17	-1	0	1739454706652	f
12	21	0	-1	0	1739454720110	f
12	19	4	0	0	1739454722935	f
12	16	0	1	0	1739454724506	f
12	22	17	1	0	1739454726518	f
12	18	17	-1	0	1739454731687	f
12	21	17	0	0	1739454735231	f
12	25	17	1	0	1739454774992	f
12	25	0	-1	0	1739454848365	f
12	26	17	1	0	1739454857492	f
12	5	28	1	0	1739455304631	f
12	4	19	0	0	1739455308645	f
12	4	23	-1	0	1739455312505	f
12	30	23	-1	0	1739455691660	f
12	31	30	1	0	1739456174574	f
12	31	28	1	0	1739456178873	f
12	32	11	1	0	1739456573277	f
9	11	34	-1	0	1739456573846	f
12	18	19	-1	0	1739457625834	f
12	10	11	-1	0	1739457637209	f
12	22	31	0	0	1739458153948	f
12	3	31	-1	0	1739458170221	f
12	5	34	-1	0	1739458889274	f
12	36	34	1	0	1739459728577	f
12	13	2	-1	0	1739459732214	f
12	41	17	1	0	1739459952778	f
12	16	6	-1	0	1739459954957	f
12	43	19	-1	0	1739459957084	f
12	43	11	1	0	1739459960139	f
12	16	31	-1	0	1739459960602	f
12	43	4	1	0	1739459964230	f
12	45	30	0	0	1739460441695	f
12	12	1	-1	0	1739461824474	f
12	51	6	-1	0	1739462326301	f
12	53	4	-1	0	1739462900282	f
12	53	7	-1	0	1739462926707	f
12	14	25	-1	0	1739462927173	f
12	53	33	-1	0	1739462934041	f
12	54	3	-1	0	1739462942482	f
12	54	11	0	0	1739462961922	f
12	54	6	-1	0	1739463007338	f
12	55	22	-1	0	1739463407398	f
12	56	36	0	0	1739464074653	f
12	56	31	-1	0	1739464080503	f
12	34	20	-1	0	1739464422796	f
12	3	36	-1	0	1739465070627	f
12	3	21	-1	0	1739465078792	f
12	18	26	-1	0	1739465555119	f
12	30	39	-1	0	1739466213813	f
12	11	26	-1	0	1739466522747	f
12	18	39	-1	0	1739480183366	f
12	60	36	1	0	1739596159095	f
12	60	22	0	0	1739596163801	f
12	60	4	-1	0	1739596178807	f
12	60	2	-1	0	1739596196032	f
12	60	39	0	0	1739596217337	f
2	47	6	1	0	1736701437158	f
2	47	30	-1	0	1736701448528	f
2	48	2	0	0	1736701484339	f
2	48	40	-1	0	1736701494586	f
2	48	6	1	0	1736701497318	f
2	49	44	-1	0	1736701502207	f
2	49	54	-1	0	1736701507965	f
2	49	30	0	0	1736701512435	f
2	49	6	-1	0	1736701515099	f
2	47	38	-1	0	1736701516439	f
2	49	38	-1	0	1736701517569	f
2	49	13	-1	0	1736701520569	f
2	47	55	-1	0	1736701523010	f
2	49	2	-1	0	1736701524307	f
2	49	25	-1	0	1736701527211	f
2	47	54	-1	0	1736701527759	f
2	49	33	-1	0	1736701531586	f
2	47	28	0	0	1736701535490	f
2	49	40	-1	0	1736701536799	f
2	49	36	-1	0	1736701541492	f
2	47	46	-1	0	1736701542277	f
2	49	56	1	0	1736701546867	f
2	49	12	-1	0	1736701551384	f
2	47	34	-1	0	1736701552259	f
2	47	49	-1	0	1736701563048	f
2	49	41	0	0	1736701571692	f
2	49	1	1	0	1736701574387	f
2	47	56	1	0	1736701577293	f
2	49	55	-1	0	1736701577884	f
2	49	20	0	0	1736701583105	f
2	49	50	0	0	1736701588429	f
2	49	9	-1	0	1736701592254	f
2	49	34	-1	0	1736701598667	f
2	47	37	-1	0	1736701602866	f
2	49	45	-1	0	1736701603167	f
2	49	46	-1	0	1736701607636	f
2	47	24	1	0	1736701611379	f
2	49	17	-1	0	1736701611988	f
2	47	13	-1	0	1736701614880	f
2	49	49	0	0	1736701622875	f
2	49	35	-1	0	1736701628256	f
2	49	10	-1	0	1736701631001	f
2	47	51	0	0	1736701632270	f
2	49	52	1	0	1736701635087	f
2	49	31	-1	0	1736701639196	f
2	47	47	-1	0	1736701641420	f
2	49	42	-1	0	1736701644026	f
2	47	21	-1	0	1736701647989	f
2	49	39	-1	0	1736701653872	f
2	47	15	-1	0	1736701654508	f
2	49	27	0	0	1736701662047	f
2	47	32	-1	0	1736701665796	f
2	49	32	-1	0	1736701669480	f
2	47	53	-1	0	1736701669745	f
2	50	2	0	0	1736701670364	f
2	49	5	0	0	1736701674150	f
2	47	35	-1	0	1736701675651	f
2	49	43	-1	0	1736701675952	f
2	50	30	-1	0	1736701679073	f
2	49	53	-1	0	1736701679449	f
2	50	6	1	0	1736701682285	f
2	49	48	-1	0	1736701684238	f
2	50	1	1	0	1736701684665	f
2	47	50	0	0	1736701687161	f
2	50	53	-1	0	1736701690190	f
2	47	42	-1	0	1736701694482	f
2	49	3	0	0	1736701697638	f
2	50	20	-1	0	1736701697673	f
2	50	46	-1	0	1736701700571	f
2	47	45	-1	0	1736701701234	f
2	47	12	-1	0	1736701706832	f
2	47	31	-1	0	1736701713383	f
2	47	27	1	0	1736701718301	f
2	49	24	-1	0	1736701719979	f
2	49	51	-1	0	1736701723500	f
2	47	44	-1	0	1736701726151	f
2	49	11	-1	0	1736701727540	f
2	47	9	-1	0	1736701728517	f
2	47	10	-1	0	1736701731522	f
2	49	29	0	0	1736701735938	f
2	47	20	-1	0	1736701741253	f
2	49	23	0	0	1736701741267	f
2	49	28	-1	0	1736701746354	f
2	49	37	-1	0	1736701751975	f
2	49	21	-1	0	1736701756033	f
2	49	47	-1	0	1736701761895	f
2	47	29	0	0	1736701762320	f
2	49	22	-1	0	1736701769976	f
2	49	15	-1	0	1736701772921	f
2	47	41	1	0	1736701783007	f
2	47	25	-1	0	1736701786629	f
2	51	30	0	0	1736701789123	f
2	51	2	1	0	1736701791585	f
2	51	46	-1	0	1736701799354	f
2	51	1	1	0	1736701800999	f
2	51	38	-1	0	1736701803882	f
2	51	49	0	0	1736701813269	f
2	51	6	1	0	1736701815293	f
2	47	57	-1	0	1736701815991	f
2	51	5	-1	0	1736701817204	f
2	51	10	-1	0	1736701820250	f
2	51	56	1	0	1736701834476	f
2	51	28	-1	0	1736701838531	f
2	51	42	-1	0	1736701843980	f
2	51	44	-1	0	1736701846754	f
2	51	33	-1	0	1736701859725	f
2	51	52	1	0	1736701863841	f
2	51	31	-1	0	1736701869285	f
2	51	57	0	0	1736701875022	f
2	47	58	-1	0	1736701878533	f
2	51	15	-1	0	1736701879287	f
2	47	22	0	0	1736701884332	f
2	51	39	-1	0	1736701884407	f
2	47	17	-1	0	1736701886847	f
2	51	43	-1	0	1736701887556	f
2	47	48	1	0	1736701891680	f
2	51	58	-1	0	1736701892673	f
2	51	9	-1	0	1736701895479	f
2	47	5	-1	0	1736701896453	f
2	51	40	-1	0	1736701900699	f
2	47	36	-1	0	1736701903113	f
2	47	23	-1	0	1736701910251	f
2	51	54	0	0	1736701918005	f
2	47	11	0	0	1736701918667	f
2	51	47	0	0	1736701938202	f
2	51	53	-1	0	1736701941501	f
2	51	13	-1	0	1736701944335	f
2	51	25	-1	0	1736701947622	f
2	51	12	-1	0	1736701955855	f
2	51	45	-1	0	1736701963658	f
2	51	34	1	0	1736701972373	f
2	51	20	0	0	1736701982702	f
2	51	41	1	0	1736701989767	f
2	51	32	-1	0	1736701995460	f
2	51	50	0	0	1736702001667	f
2	51	55	-1	0	1736702008585	f
2	52	2	0	0	1736702009868	f
2	51	36	-1	0	1736702026123	f
2	76	24	0	0	1736742971764	f
4	1	5	-1	0	1736756101127	f
4	1	4	-1	0	1736756107372	f
4	1	2	-1	0	1736756116183	f
5	1	14	-1	0	1736794575388	f
5	4	15	-1	0	1736794601770	f
5	9	8	-1	0	1736795948713	f
5	9	10	0	0	1736795951600	f
5	9	12	0	0	1736795960464	f
5	13	16	1	0	1736802889719	f
5	13	6	0	0	1736802930656	f
5	9	21	-1	0	1736808499265	f
5	20	16	-1	0	1736814464040	f
5	20	1	0	0	1736814466260	f
5	20	11	0	0	1736814475084	f
5	20	21	-1	0	1736814490631	f
5	20	12	0	0	1736814527889	f
5	20	17	-1	0	1736814548204	f
5	20	18	0	0	1736814557075	f
5	20	7	-1	0	1736814591545	f
5	20	19	-1	0	1736814630916	f
2	78	37	-1	0	1736827617830	f
2	80	1	1	0	1736879802992	f
2	82	38	0	0	1736880541961	f
2	82	68	-1	0	1736880571359	f
2	82	61	-1	0	1736880875080	f
2	86	1	-1	0	1736882274448	f
2	86	34	-1	0	1736882288352	f
2	87	34	0	0	1736916237343	f
2	87	43	-1	0	1736916243832	f
2	87	31	-1	0	1736916251641	f
2	87	28	1	0	1736916267008	f
2	87	42	-1	0	1736916664061	f
2	88	33	-1	0	1736994596651	f
2	88	37	-1	0	1736995372704	f
2	89	25	-1	0	1737002535436	f
2	89	59	-1	0	1737002551951	f
2	89	23	1	0	1737002568431	f
2	90	63	-1	0	1737041493076	f
2	90	60	-1	0	1737041507293	f
2	90	58	1	0	1737041896412	f
2	90	62	-1	0	1737041928640	f
2	90	24	1	0	1737041950000	f
2	92	66	-1	0	1737467379487	f
2	92	43	-1	0	1737467386287	f
2	92	27	-1	0	1737467396477	f
2	92	40	-1	0	1737467422421	f
2	92	35	-1	0	1737467433728	f
2	92	28	-1	0	1737467447698	f
2	92	34	-1	0	1737467452204	f
2	92	33	-1	0	1737467463652	f
2	92	72	-1	0	1737467470124	f
2	92	32	-1	0	1737467482780	f
2	92	39	1	0	1737467501832	f
2	92	47	-1	0	1737467514468	f
2	92	58	1	0	1737467531005	f
2	92	44	-1	0	1737467538190	f
11	7	1	0	0	1739296989813	f
11	11	5	-1	0	1739308729645	f
11	11	12	-1	0	1739351728053	f
12	0	1	-1	0	1739381958832	f
9	8	29	-1	0	1739453811408	f
12	14	17	1	0	1739454686963	f
12	11	0	1	0	1739454689164	f
12	15	17	-1	0	1739454694855	f
12	17	4	-1	0	1739454699301	f
12	9	23	-1	0	1739455333718	f
12	4	8	1	0	1739455719577	f
12	31	27	0	0	1739456184290	f
12	26	29	1	0	1739456216006	f
12	9	8	1	0	1739456228213	f
12	32	0	-1	0	1739456583663	f
12	9	7	0	0	1739457647572	f
12	34	32	0	0	1739458270124	f
12	9	2	-1	0	1739459070855	f
12	9	33	-1	0	1739459079222	f
12	36	2	-1	0	1739459731779	f
12	36	32	0	0	1739459737090	f
12	35	31	-1	0	1739459739355	f
12	36	6	-1	0	1739459741630	f
12	36	0	1	0	1739459747384	f
12	35	19	0	0	1739459748592	f
12	13	34	-1	0	1739459748672	f
12	43	8	1	0	1739459968276	f
12	41	7	1	0	1739459972873	f
12	16	3	-1	0	1739459973508	f
12	41	11	1	0	1739459977868	f
12	45	1	0	0	1739460486771	f
12	45	13	-1	0	1739460538421	f
12	47	6	-1	0	1739461833754	f
12	51	7	-1	0	1739462339588	f
12	51	4	-1	0	1739462348812	f
12	51	30	-1	0	1739462367081	f
12	53	11	1	0	1739462907267	f
12	31	2	-1	0	1739462918813	f
12	41	14	-1	0	1739462923387	f
12	54	2	0	0	1739462923973	f
12	31	14	1	0	1739462925664	f
12	5	22	-1	0	1739463409999	f
12	39	25	-1	0	1739464091042	f
12	15	20	-1	0	1739464111902	f
12	53	6	-1	0	1739464115935	f
12	39	14	-1	0	1739464131191	f
12	21	14	-1	0	1739464135044	f
12	12	36	-1	0	1739464137133	f
12	29	22	-1	0	1739464137244	f
12	29	6	-1	0	1739464143494	f
12	35	17	-1	0	1739464148418	f
12	20	22	-1	0	1739464148605	f
12	30	22	-1	0	1739464152623	f
12	30	20	-1	0	1739464160984	f
12	16	36	-1	0	1739464689697	f
12	22	18	-1	0	1739465070629	f
12	18	37	-1	0	1739465562853	f
12	30	38	1	0	1739466219750	f
12	21	39	-1	0	1739467342727	f
12	18	26	-1	0	1739480968671	f
12	60	34	-1	0	1739596170467	f
12	60	31	-1	0	1739596182854	f
12	60	13	0	0	1739596188324	f
12	60	27	1	0	1739596192743	f
12	60	20	0	0	1739596201594	f
2	51	35	-1	0	1736702014964	f
2	51	29	0	0	1736702049081	f
2	51	37	1	0	1736702074209	f
2	51	24	1	0	1736702114274	f
2	13	58	1	0	1736702205859	f
2	76	50	0	0	1736742974361	f
2	76	73	-1	0	1736743015745	f
5	0	0	0	0	1736789772253	f
5	1	13	-1	0	1736794584826	f
5	9	16	0	0	1736795950446	f
5	9	17	0	0	1736795959240	f
5	9	2	0	0	1736795961694	f
5	13	10	0	0	1736802906247	f
5	9	22	-1	0	1736808503732	f
5	20	3	0	0	1736814468870	f
5	20	9	0	0	1736814473903	f
5	20	8	-1	0	1736814482011	f
5	20	5	-1	0	1736814533573	f
5	20	20	-1	0	1736814537315	f
2	78	35	0	0	1736827626977	f
2	79	46	-1	0	1736879816953	f
2	82	23	-1	0	1736880547410	f
2	82	58	1	0	1736880555619	f
2	82	73	-1	0	1736880560408	f
2	82	12	-1	0	1736880903293	f
2	86	2	0	0	1736882282437	f
2	86	6	1	0	1736882294879	f
2	86	10	-1	0	1736882299263	f
2	87	15	-1	0	1736916241789	f
2	87	56	-1	0	1736917057003	f
2	88	70	-1	0	1736994600266	f
2	88	24	1	0	1736995391630	f
2	89	30	-1	0	1737002538005	f
2	90	40	-1	0	1737041497361	f
2	90	61	1	0	1737041903768	f
2	90	49	1	0	1737041915214	f
2	92	6	1	0	1737467381036	f
2	92	70	-1	0	1737467401871	f
2	92	46	-1	0	1737467408804	f
2	92	55	-1	0	1737467417086	f
2	92	13	-1	0	1737467438203	f
2	92	5	-1	0	1737467442816	f
2	92	42	-1	0	1737467454989	f
2	92	12	-1	0	1737467457500	f
2	92	63	-1	0	1737467474089	f
2	92	30	-1	0	1737467484873	f
11	8	1	0	0	1739296989991	f
11	11	6	-1	0	1739308763331	f
11	11	13	-1	0	1739351744800	f
12	0	2	-1	0	1739382030967	f
12	0	3	-1	0	1739382037877	f
12	0	4	-1	0	1739382045268	f
12	0	10	-1	0	1739382097654	f
9	9	29	0	0	1739453994102	f
12	18	4	-1	0	1739454721010	f
12	18	17	-1	0	1739454731772	f
12	24	4	-1	0	1739454751640	f
12	23	17	-1	0	1739454751871	f
12	9	27	0	0	1739455350373	f
12	4	29	0	0	1739455725166	f
12	31	8	1	0	1739456187429	f
12	31	4	-1	0	1739456193829	f
9	11	35	-1	0	1739456650431	f
9	8	38	-1	0	1739457711330	f
12	33	6	0	0	1739457747255	f
12	33	31	-1	0	1739457756218	f
12	18	31	-1	0	1739457766563	f
12	11	31	-1	0	1739457771247	f
12	33	30	1	0	1739457789286	f
12	33	17	0	0	1739457796757	f
9	12	33	-1	0	1739457806432	f
9	12	35	-1	0	1739457811350	f
9	12	34	-1	0	1739457816027	f
12	33	11	1	0	1739457848549	f
12	4	31	-1	0	1739458346738	f
12	9	3	-1	0	1739459074177	f
12	13	1	-1	0	1739459737919	f
12	36	17	-1	0	1739459783798	f
12	43	32	1	0	1739459972627	f
12	41	8	1	0	1739459999324	f
12	45	3	0	0	1739460555114	f
12	39	13	-1	0	1739461835179	f
12	3	23	-1	0	1739461845112	f
12	39	0	0	0	1739461848284	f
12	49	13	-1	0	1739461878734	f
12	50	19	-1	0	1739461915051	f
12	49	7	-1	0	1739461915114	f
12	49	6	-1	0	1739461927513	f
12	49	31	-1	0	1739461930559	f
12	49	30	0	0	1739461938166	f
12	49	29	-1	0	1739461959238	f
12	51	29	-1	0	1739462358297	f
12	54	0	1	0	1739462912140	f
12	11	22	-1	0	1739463484512	f
12	18	36	0	0	1739464094706	f
12	36	22	-1	0	1739464105238	f
12	41	20	-1	0	1739464108476	f
12	49	35	-1	0	1739464109704	f
12	36	20	1	0	1739464109716	f
12	41	35	-1	0	1739464111009	f
12	36	35	-1	0	1739464115819	f
12	15	22	0	0	1739464124557	f
12	56	11	0	0	1739464133083	f
12	35	0	1	0	1739464136802	f
12	53	35	-1	0	1739464141905	f
12	21	20	0	0	1739464142532	f
12	35	13	-1	0	1739464142582	f
12	10	35	-1	0	1739464155736	f
12	15	25	-1	0	1739464158351	f
12	39	34	1	0	1739464161360	f
12	15	13	-1	0	1739464163556	f
12	57	29	1	0	1739464168434	f
12	56	1	-1	0	1739464169183	f
12	39	17	-1	0	1739464181678	f
12	57	0	-1	0	1739464183891	f
12	39	8	-1	0	1739464188668	f
12	58	0	1	0	1739464199803	f
12	35	35	-1	0	1739464200136	f
12	10	22	-1	0	1739464200752	f
12	15	7	-1	0	1739464200867	f
12	20	25	-1	0	1739464205373	f
12	10	20	-1	0	1739464205420	f
12	53	3	-1	0	1739464213232	f
12	57	27	-1	0	1739464226194	f
12	35	11	0	0	1739464243572	f
12	15	34	-1	0	1739464245852	f
12	29	31	-1	0	1739464253874	f
12	40	20	1	0	1739464254875	f
12	56	4	-1	0	1739464280983	f
12	31	20	-1	0	1739464282668	f
12	57	22	-1	0	1739464287820	f
12	29	11	1	0	1739464295544	f
12	35	6	-1	0	1739464296574	f
12	57	13	1	0	1739464303072	f
12	29	32	1	0	1739464323533	f
12	16	20	-1	0	1739464700914	f
2	51	21	-1	0	1736702021408	f
2	51	48	1	0	1736702031388	f
2	51	17	-1	0	1736702052757	f
2	51	22	0	0	1736702062772	f
2	51	3	-1	0	1736702066792	f
2	51	27	0	0	1736702080837	f
2	51	51	1	0	1736702091336	f
2	51	23	-1	0	1736702097771	f
2	51	11	-1	0	1736702118277	f
2	13	57	-1	0	1736702201855	f
2	53	30	-1	0	1736702311393	f
2	52	45	-1	0	1736702359934	f
2	52	46	0	0	1736702369786	f
2	52	6	0	0	1736702375073	f
2	52	30	1	0	1736702379935	f
2	52	52	0	0	1736702383952	f
2	52	1	0	0	1736702386713	f
2	52	38	1	0	1736702396212	f
2	52	25	-1	0	1736702404652	f
2	52	11	1	0	1736702421921	f
2	53	57	-1	0	1736702428725	f
2	51	59	-1	0	1736702429778	f
2	53	43	-1	0	1736702432595	f
2	53	6	-1	0	1736702434874	f
2	52	56	0	0	1736702435144	f
2	52	20	-1	0	1736702440583	f
2	53	58	-1	0	1736702443342	f
2	52	31	0	0	1736702451363	f
2	52	58	0	0	1736702455973	f
2	53	2	-1	0	1736702459158	f
2	52	59	0	0	1736702462694	f
2	53	1	1	0	1736702462987	f
2	52	55	-1	0	1736702468105	f
2	52	27	-1	0	1736702480145	f
2	53	56	-1	0	1736702484975	f
2	52	57	0	0	1736702485573	f
2	52	3	-1	0	1736702491813	f
2	52	15	-1	0	1736702496544	f
2	53	5	-1	0	1736702500422	f
2	52	47	-1	0	1736702502193	f
2	53	40	-1	0	1736702505238	f
2	52	54	-1	0	1736702510061	f
2	53	39	-1	0	1736702514381	f
2	52	53	1	0	1736702522038	f
2	53	51	-1	0	1736702530863	f
2	52	35	1	0	1736702533213	f
2	53	12	-1	0	1736702535363	f
2	52	12	-1	0	1736702538163	f
2	53	45	-1	0	1736702539965	f
2	52	33	1	0	1736702548437	f
2	53	37	-1	0	1736702554324	f
2	52	29	-1	0	1736702554547	f
2	52	28	0	0	1736702558813	f
2	53	25	-1	0	1736702559646	f
2	53	50	-1	0	1736702568055	f
2	52	13	0	0	1736702571944	f
2	53	55	-1	0	1736702572140	f
2	52	43	0	0	1736702576283	f
2	52	48	-1	0	1736702582234	f
2	53	22	1	0	1736702586576	f
2	53	47	-1	0	1736702597443	f
2	53	31	-1	0	1736702602178	f
2	53	59	1	0	1736702612548	f
2	53	13	-1	0	1736702615268	f
2	53	44	-1	0	1736702617691	f
2	53	53	-1	0	1736702621400	f
2	53	54	-1	0	1736702626049	f
2	53	49	-1	0	1736702631072	f
2	53	17	-1	0	1736702634348	f
2	53	42	-1	0	1736702641107	f
2	53	52	1	0	1736702649866	f
2	53	38	-1	0	1736702657006	f
2	53	46	-1	0	1736702660545	f
2	53	20	-1	0	1736702664141	f
2	53	33	-1	0	1736702669077	f
2	53	35	-1	0	1736702673952	f
2	53	36	-1	0	1736702681271	f
2	53	34	-1	0	1736702688130	f
2	53	28	-1	0	1736702696043	f
2	53	21	-1	0	1736702702662	f
2	53	27	-1	0	1736702711767	f
2	53	48	-1	0	1736702717234	f
2	53	15	-1	0	1736702720911	f
2	53	29	-1	0	1736702727212	f
2	53	23	-1	0	1736702732325	f
2	52	60	-1	0	1736702737534	f
2	53	11	-1	0	1736702737870	f
2	53	10	-1	0	1736702741965	f
2	53	9	-1	0	1736702743706	f
2	52	40	0	0	1736702748161	f
2	53	60	-1	0	1736702748412	f
2	53	3	-1	0	1736702753279	f
2	52	39	-1	0	1736702758325	f
2	53	41	1	0	1736702775893	f
2	53	24	1	0	1736702785519	f
2	53	32	-1	0	1736702799857	f
2	13	59	0	0	1736702966144	f
2	52	42	-1	0	1736703041771	f
2	52	10	-1	0	1736703049199	f
2	52	36	-1	0	1736703053892	f
2	52	51	0	0	1736703089082	f
2	52	17	-1	0	1736703096132	f
2	52	5	-1	0	1736703100081	f
2	52	41	-1	0	1736703129662	f
2	52	32	-1	0	1736703146749	f
2	52	44	0	0	1736703179863	f
2	52	22	1	0	1736703187114	f
2	54	2	-1	0	1736703188193	f
2	52	37	-1	0	1736703191640	f
2	54	1	1	0	1736703198738	f
2	54	30	-1	0	1736703202234	f
2	54	6	1	0	1736703204322	f
2	54	38	0	0	1736703207132	f
2	52	34	1	0	1736703210040	f
2	54	3	-1	0	1736703211430	f
2	52	23	-1	0	1736703214832	f
2	52	9	0	0	1736703223482	f
2	52	21	-1	0	1736703236524	f
2	52	50	0	0	1736703242831	f
2	52	49	0	0	1736703245602	f
2	52	24	0	0	1736703255883	f
2	55	2	0	0	1736703595015	f
2	55	3	-1	0	1736703598707	f
2	55	1	1	0	1736703601285	f
2	55	38	-1	0	1736703607230	f
2	55	6	1	0	1736703608907	f
2	55	60	0	0	1736703621304	f
2	55	46	-1	0	1736703623921	f
2	55	44	-1	0	1736703627256	f
2	55	30	-1	0	1736703633262	f
2	55	5	-1	0	1736703635995	f
2	55	58	-1	0	1736703646649	f
2	55	42	-1	0	1736703649571	f
2	55	33	-1	0	1736703653173	f
2	55	57	0	0	1736703661404	f
2	55	52	0	0	1736703665046	f
2	55	37	0	0	1736703684050	f
2	55	9	-1	0	1736703687808	f
2	55	13	-1	0	1736703692290	f
2	55	55	-1	0	1736703695877	f
2	55	17	-1	0	1736703698862	f
2	55	49	0	0	1736703706804	f
2	55	27	-1	0	1736703712492	f
2	1	12	0	0	1736703786875	f
2	52	61	-1	0	1736704545216	f
2	56	6	0	0	1736708201431	f
2	56	30	-1	0	1736708208219	f
2	56	2	0	0	1736708214394	f
2	56	43	-1	0	1736708216857	f
2	56	61	0	0	1736708220514	f
2	56	44	-1	0	1736708226261	f
2	56	1	0	0	1736708227976	f
2	56	58	-1	0	1736708240491	f
2	56	39	-1	0	1736708254476	f
2	56	56	1	0	1736708279233	f
2	56	38	-1	0	1736708283034	f
2	56	36	-1	0	1736708288446	f
2	56	46	-1	0	1736708292455	f
2	56	48	1	0	1736708310459	f
2	56	59	-1	0	1736708317774	f
2	56	42	-1	0	1736708324936	f
2	56	31	-1	0	1736708332341	f
2	56	47	-1	0	1736708336376	f
2	56	40	-1	0	1736708340994	f
2	56	12	-1	0	1736708344766	f
2	56	53	-1	0	1736708349076	f
2	56	34	-1	0	1736708358896	f
2	56	33	-1	0	1736708367726	f
2	56	35	-1	0	1736708378408	f
2	56	51	1	0	1736708387347	f
2	56	60	-1	0	1736708403085	f
2	56	22	-1	0	1736708406126	f
2	56	21	-1	0	1736708414850	f
2	56	55	-1	0	1736708418809	f
2	56	57	0	0	1736708424226	f
2	56	20	0	0	1736708433797	f
2	56	9	-1	0	1736708439656	f
2	56	17	-1	0	1736708442516	f
2	56	27	-1	0	1736708453844	f
2	56	5	-1	0	1736708461201	f
2	56	54	-1	0	1736708471361	f
2	56	28	0	0	1736708482940	f
2	56	29	-1	0	1736708488801	f
2	56	49	0	0	1736708495425	f
2	56	41	0	0	1736708518381	f
2	56	10	-1	0	1736708520901	f
2	56	15	-1	0	1736708524180	f
2	56	32	-1	0	1736708537821	f
2	56	13	-1	0	1736708543071	f
2	56	11	0	0	1736708553126	f
2	56	23	-1	0	1736708571480	f
2	56	25	-1	0	1736708574799	f
2	56	3	-1	0	1736708581322	f
2	56	50	0	0	1736708585379	f
2	56	52	-1	0	1736708589281	f
2	56	37	1	0	1736708602561	f
2	56	45	-1	0	1736708620860	f
2	56	24	-1	0	1736708627640	f
2	57	59	-1	0	1736708752177	f
2	57	1	1	0	1736708777568	f
2	57	60	-1	0	1736708814067	f
2	57	6	1	0	1736708817227	f
2	57	56	0	0	1736708831872	f
2	57	23	0	0	1736708841387	f
2	57	2	0	0	1736708847125	f
2	57	30	1	0	1736708852389	f
2	57	58	0	0	1736708867027	f
2	57	25	-1	0	1736708870309	f
2	57	53	-1	0	1736708873028	f
2	57	61	-1	0	1736708877062	f
2	57	40	-1	0	1736708880329	f
2	56	62	-1	0	1736708882064	f
2	57	12	-1	0	1736708883506	f
2	57	9	-1	0	1736708885268	f
2	57	5	0	0	1736708889867	f
2	57	39	-1	0	1736708898468	f
2	57	35	-1	0	1736708903987	f
2	57	54	-1	0	1736708907287	f
2	57	62	0	0	1736708913407	f
2	57	42	-1	0	1736708917509	f
2	57	57	0	0	1736708921407	f
2	57	36	0	0	1736708927527	f
2	57	22	0	0	1736708933252	f
2	57	45	-1	0	1736708950727	f
2	57	31	-1	0	1736708955267	f
2	57	17	-1	0	1736708957870	f
2	57	48	-1	0	1736708968599	f
2	57	38	-1	0	1736708972308	f
2	57	37	-1	0	1736708981087	f
2	57	43	-1	0	1736708983150	f
2	57	13	-1	0	1736708987887	f
2	57	41	-1	0	1736709000707	f
2	57	10	-1	0	1736709004747	f
2	57	33	0	0	1736709017908	f
2	57	44	-1	0	1736709020257	f
2	57	34	-1	0	1736709047797	f
2	57	28	-1	0	1736709054427	f
2	57	46	-1	0	1736709058425	f
2	57	27	-1	0	1736709079767	f
2	57	47	-1	0	1736709085808	f
2	57	21	-1	0	1736709094307	f
2	57	20	0	0	1736709099888	f
2	57	24	-1	0	1736709106397	f
2	57	51	-1	0	1736709110287	f
2	57	11	-1	0	1736709116431	f
2	57	55	-1	0	1736709122178	f
2	57	50	0	0	1736709126844	f
2	57	49	1	0	1736709132187	f
2	57	15	-1	0	1736709137989	f
2	57	29	0	0	1736709149726	f
2	57	3	-1	0	1736709157106	f
2	0	63	0	0	1736709159375	f
2	57	32	0	0	1736709163330	f
2	57	63	-1	0	1736709168047	f
2	57	52	1	0	1736709173131	f
2	58	34	-1	0	1736717867290	f
2	58	30	1	0	1736717875649	f
2	58	2	0	0	1736717885335	f
2	58	56	1	0	1736717894509	f
2	58	61	1	0	1736717909325	f
2	58	1	1	0	1736717913753	f
2	58	6	1	0	1736717920371	f
2	58	63	-1	0	1736717923789	f
2	58	48	0	0	1736717931217	f
2	59	30	1	0	1736719950197	f
2	59	31	-1	0	1736719955844	f
2	59	17	-1	0	1736719958348	f
2	59	63	-1	0	1736719960995	f
2	59	61	-1	0	1736719964617	f
2	59	48	-1	0	1736719969589	f
2	59	2	0	0	1736719973342	f
2	59	1	0	0	1736719976394	f
2	59	38	0	0	1736719978703	f
2	59	6	0	0	1736719985270	f
2	59	28	-1	0	1736719990036	f
2	12	72	-1	0	1736743875754	f
5	0	1	0	0	1736789783699	f
5	1	15	-1	0	1736794607627	f
5	8	18	-1	0	1736796013775	f
5	13	1	0	0	1736802932675	f
5	13	9	-1	0	1736802939009	f
5	13	2	0	0	1736802941636	f
5	17	1	-1	0	1736809741560	f
2	78	1	1	0	1736826738934	f
2	78	2	0	0	1736826750791	f
2	78	21	-1	0	1736827638714	f
2	79	60	-1	0	1736879839875	f
2	82	13	1	0	1736880565391	f
2	82	71	1	0	1736880584458	f
2	82	56	0	0	1736880613313	f
2	82	62	0	0	1736880634349	f
2	82	3	-1	0	1736880913540	f
2	86	67	-1	0	1736882292612	f
2	86	13	-1	0	1736882297118	f
2	86	25	-1	0	1736882301196	f
2	86	73	-1	0	1736882308907	f
2	86	47	-1	0	1736882325652	f
2	86	48	1	0	1736882335305	f
2	87	5	-1	0	1736916248606	f
2	87	37	-1	0	1736917071308	f
2	88	30	-1	0	1736994623410	f
2	88	37	-1	0	1736996045346	f
2	88	61	-1	0	1736996051820	f
2	89	46	-1	0	1737002556626	f
2	89	56	-1	0	1737002579571	f
2	90	33	-1	0	1737041524641	f
2	90	11	0	0	1737041540237	f
2	90	52	-1	0	1737041918967	f
2	90	32	-1	0	1737041933064	f
2	92	67	-1	0	1737467406013	f
2	92	10	-1	0	1737467420103	f
2	92	38	-1	0	1737467424375	f
2	92	3	-1	0	1737467435775	f
2	92	73	-1	0	1737467441002	f
2	92	15	-1	0	1737467444685	f
2	92	51	-1	0	1737467466957	f
2	92	17	-1	0	1737467472633	f
2	92	56	1	0	1737467498072	f
2	92	23	-1	0	1737467510742	f
2	92	41	-1	0	1737467520798	f
2	92	21	-1	0	1737467525631	f
2	92	11	-1	0	1737467534191	f
2	92	59	1	0	1737467551415	f
2	92	54	-1	0	1737467568441	f
2	92	25	-1	0	1737467585144	f
2	92	36	-1	0	1737467603893	f
11	8	0	0	0	1739296990912	f
11	11	7	-1	0	1739316118551	f
11	14	10	-1	0	1739362870292	f
12	0	5	-1	0	1739382053081	f
12	0	7	-1	0	1739382073497	f
12	0	8	-1	0	1739382082620	f
12	0	9	-1	0	1739382090311	f
9	9	30	-1	0	1739454039938	f
12	23	0	1	0	1739454744889	f
12	9	28	0	0	1739455359909	f
12	2	29	1	0	1739455731886	f
12	30	4	-1	0	1739455731924	f
12	30	19	-1	0	1739455765025	f
12	11	8	-1	0	1739455765713	f
12	31	19	0	0	1739456201740	f
12	30	30	1	0	1739456713804	f
12	5	7	1	0	1739457726362	f
12	34	31	-1	0	1739458349634	f
12	34	29	-1	0	1739458401275	f
12	9	34	1	0	1739459087850	f
12	20	1	-1	0	1739459748266	f
12	20	34	-1	0	1739459754886	f
12	36	23	-1	0	1739459758601	f
12	20	13	-1	0	1739459763458	f
12	13	23	1	0	1739459781040	f
12	22	2	-1	0	1739459801120	f
12	13	11	1	0	1739459802894	f
12	40	1	1	0	1739459805372	f
12	7	13	-1	0	1739459808047	f
12	38	2	0	0	1739459809059	f
12	22	3	-1	0	1739459815222	f
12	38	23	-1	0	1739459834727	f
12	7	19	0	0	1739459838347	f
12	7	6	-1	0	1739459878480	f
12	16	23	-1	0	1739459892341	f
12	16	2	0	0	1739459900106	f
12	7	32	1	0	1739459900692	f
12	41	2	-1	0	1739459903397	f
12	7	28	1	0	1739459905392	f
12	43	23	-1	0	1739459909587	f
12	38	6	-1	0	1739459910118	f
12	14	30	-1	0	1739459912783	f
12	40	3	-1	0	1739459974096	f
12	14	33	0	0	1739459976579	f
12	41	0	-1	0	1739459983890	f
12	16	28	1	0	1739460011877	f
12	27	23	-1	0	1739460036744	f
12	44	34	0	0	1739460054262	f
12	27	2	-1	0	1739460055863	f
9	13	36	-1	0	1739460658392	f
9	13	33	-1	0	1739460669250	f
12	47	27	-1	0	1739461837424	f
12	47	0	-1	0	1739461838815	f
12	51	32	0	0	1739462374839	f
12	51	34	1	0	1739462379752	f
12	51	8	0	0	1739462387574	f
12	41	25	-1	0	1739462912230	f
12	53	13	-1	0	1739462916524	f
12	11	1	-1	0	1739463489407	f
12	34	35	-1	0	1739463496505	f
12	11	2	-1	0	1739463509575	f
12	39	22	-1	0	1739464096234	f
12	39	35	-1	0	1739464102086	f
12	41	22	-1	0	1739464102669	f
12	15	6	0	0	1739464107590	f
12	49	20	0	0	1739464116533	f
12	39	31	0	0	1739464117330	f
12	15	23	-1	0	1739464134369	f
12	53	17	-1	0	1739464135188	f
12	16	22	-1	0	1739464708793	f
12	27	14	-1	0	1739465072407	f
12	35	18	-1	0	1739465074156	f
12	22	21	1	0	1739465081799	f
12	18	18	-1	0	1739465571299	f
12	34	39	0	0	1739466239289	f
12	34	38	0	0	1739466254335	f
12	21	38	-1	0	1739467345784	f
12	21	40	0	0	1739534278832	f
12	60	29	1	0	1739596234844	f
2	59	60	-1	0	1736719983016	f
2	59	64	-1	0	1736720059498	f
2	59	65	-1	0	1736720096363	f
2	59	59	0	0	1736720103016	f
2	59	58	1	0	1736720107985	f
2	59	33	1	0	1736720112516	f
2	59	42	-1	0	1736720118233	f
2	59	15	-1	0	1736720121065	f
2	59	9	-1	0	1736720122325	f
2	59	62	0	0	1736720126662	f
2	59	13	0	0	1736720230177	f
2	59	56	-1	0	1736720235066	f
2	59	25	0	0	1736720238532	f
2	59	53	-1	0	1736720241191	f
2	59	20	-1	0	1736720243181	f
2	59	51	-1	0	1736720245345	f
2	59	44	-1	0	1736720248598	f
2	59	55	-1	0	1736720251697	f
2	59	39	0	0	1736720256005	f
2	16	58	1	0	1736720771791	f
2	16	63	-1	0	1736720774935	f
2	16	35	-1	0	1736720783147	f
2	16	64	0	0	1736720788604	f
2	16	61	0	0	1736720791589	f
2	16	65	0	0	1736720795280	f
2	16	46	-1	0	1736720798129	f
2	16	60	-1	0	1736720802065	f
2	16	39	-1	0	1736720810971	f
2	16	40	-1	0	1736720814638	f
2	16	56	0	0	1736720824136	f
2	16	42	-1	0	1736720828014	f
2	16	34	-1	0	1736720834950	f
2	16	50	-1	0	1736720839249	f
2	16	47	-1	0	1736720844321	f
2	16	59	-1	0	1736720851495	f
2	16	57	0	0	1736720856111	f
2	16	44	-1	0	1736720858504	f
2	16	55	-1	0	1736720862139	f
2	16	51	0	0	1736720868078	f
2	16	38	-1	0	1736720869442	f
2	16	53	0	0	1736720877446	f
2	16	33	1	0	1736720882348	f
2	16	45	-1	0	1736720896430	f
2	16	43	-1	0	1736720898525	f
2	16	62	0	0	1736720912657	f
2	16	54	-1	0	1736720918312	f
2	16	49	0	0	1736720923876	f
2	16	37	-1	0	1736720933827	f
2	16	36	-1	0	1736720939974	f
2	16	48	1	0	1736720947347	f
2	16	52	-1	0	1736720950606	f
2	16	41	0	0	1736720962172	f
2	59	27	-1	0	1736721238044	f
2	59	54	-1	0	1736721246270	f
2	59	21	-1	0	1736721251019	f
2	59	35	0	0	1736721257689	f
2	59	43	0	0	1736721260429	f
2	59	5	-1	0	1736721263124	f
2	59	34	1	0	1736721268190	f
2	59	22	1	0	1736721272780	f
2	59	32	-1	0	1736721280000	f
2	59	40	0	0	1736721285660	f
2	59	45	-1	0	1736721288399	f
2	59	47	0	0	1736721292719	f
2	59	46	-1	0	1736721298796	f
2	59	52	-1	0	1736721306652	f
2	59	3	-1	0	1736721309622	f
2	59	12	0	0	1736721312615	f
2	59	37	-1	0	1736721315438	f
2	59	50	0	0	1736721318699	f
2	59	10	-1	0	1736721321109	f
2	59	29	-1	0	1736721326515	f
2	59	36	0	0	1736721331594	f
2	59	57	0	0	1736721334333	f
2	59	11	0	0	1736721339179	f
2	59	41	-1	0	1736721346822	f
2	59	23	-1	0	1736721349095	f
2	59	49	0	0	1736721351928	f
2	59	24	0	0	1736721356453	f
2	59	66	-1	0	1736721388336	f
2	59	67	-1	0	1736721430204	f
2	59	68	-1	0	1736722158717	f
2	16	66	-1	0	1736724723108	f
2	16	68	-1	0	1736724728450	f
2	16	67	0	0	1736724744898	f
2	34	30	0	0	1736727242451	f
2	34	2	0	0	1736727249792	f
2	34	1	0	0	1736727251987	f
2	60	2	-1	0	1736732431789	f
2	60	1	1	0	1736732434214	f
2	60	6	1	0	1736732435774	f
2	60	60	-1	0	1736732443817	f
2	60	30	-1	0	1736732448409	f
2	60	65	-1	0	1736732452818	f
2	60	42	-1	0	1736732466875	f
2	60	58	-1	0	1736732474541	f
2	60	22	-1	0	1736732478445	f
2	60	34	-1	0	1736732482560	f
2	60	21	-1	0	1736732487926	f
2	60	66	-1	0	1736732490774	f
2	60	68	-1	0	1736732498150	f
2	60	47	0	0	1736732513338	f
2	60	61	0	0	1736732517268	f
2	60	38	1	0	1736732521586	f
2	60	25	0	0	1736732528223	f
2	60	20	0	0	1736732534861	f
2	60	67	1	0	1736732546492	f
2	60	59	-1	0	1736732550703	f
2	60	33	-1	0	1736732565430	f
2	60	55	-1	0	1736732571730	f
2	60	12	0	0	1736732630665	f
2	60	62	-1	0	1736732639610	f
2	60	40	0	0	1736732650485	f
2	60	63	-1	0	1736732655432	f
2	60	37	0	0	1736732662287	f
2	60	52	-1	0	1736732671554	f
2	60	43	0	0	1736732681265	f
2	60	48	1	0	1736732694933	f
2	60	46	-1	0	1736732701600	f
2	60	56	0	0	1736732709251	f
2	60	64	-1	0	1736732714432	f
2	60	10	-1	0	1736732719910	f
2	60	51	0	0	1736732724986	f
2	60	53	-1	0	1736732728792	f
2	60	35	-1	0	1736732738293	f
2	60	23	1	0	1736732744494	f
2	60	13	-1	0	1736732750944	f
2	60	27	1	0	1736732758341	f
2	60	39	-1	0	1736732765309	f
2	60	44	0	0	1736732774663	f
2	60	15	-1	0	1736732777880	f
2	60	31	-1	0	1736732781160	f
2	60	3	0	0	1736732784633	f
2	60	36	-1	0	1736732792049	f
2	60	45	-1	0	1736732797252	f
2	60	57	1	0	1736732802085	f
2	60	54	-1	0	1736732807906	f
2	60	11	0	0	1736732815973	f
2	60	28	-1	0	1736732821644	f
2	60	32	-1	0	1736732831226	f
2	60	29	0	0	1736732838747	f
2	60	5	1	0	1736732845214	f
2	60	17	-1	0	1736732847479	f
2	60	50	-1	0	1736732868299	f
2	77	6	1	0	1736744241323	f
2	77	30	1	0	1736744245647	f
2	77	73	0	0	1736744269020	f
5	0	2	0	0	1736789794334	f
5	0	13	-1	0	1736794790332	f
5	8	19	-1	0	1736796202086	f
5	13	11	0	0	1736802934387	f
5	17	16	1	0	1736809743317	f
5	17	9	-1	0	1736809749733	f
5	17	15	-1	0	1736809764932	f
5	17	17	0	0	1736809774737	f
5	17	13	-1	0	1736809785813	f
5	17	19	0	0	1736809809680	f
2	78	23	-1	0	1736826743537	f
2	78	10	-1	0	1736827643874	f
2	79	73	0	0	1736879843992	f
2	79	39	-1	0	1736879853511	f
2	79	12	-1	0	1736879880014	f
2	81	66	-1	0	1736879889121	f
2	79	10	0	0	1736879894415	f
2	81	5	0	0	1736879899651	f
2	82	15	-1	0	1736880591663	f
2	82	35	1	0	1736880618871	f
2	82	29	1	0	1736880933672	f
2	82	47	-1	0	1736880962911	f
2	86	33	-1	0	1736882305940	f
2	86	38	-1	0	1736882310649	f
2	86	45	-1	0	1736882356041	f
2	87	30	-1	0	1736916279479	f
2	87	47	-1	0	1736917084331	f
2	88	59	-1	0	1736994658274	f
2	88	63	-1	0	1736996049537	f
2	89	21	-1	0	1737002559063	f
2	89	12	-1	0	1737002561814	f
2	90	54	-1	0	1737041528986	f
2	13	69	-1	0	1737062884671	f
2	13	71	0	0	1737062894410	f
2	92	65	-1	0	1737467506843	f
2	92	69	-1	0	1737467523286	f
11	12	5	-1	0	1739316195978	f
11	15	11	-1	0	1739363141590	f
12	0	6	-1	0	1739382061115	f
12	2	0	1	0	1739454339184	f
12	22	4	0	0	1739454779915	f
12	19	0	1	0	1739454789126	f
12	9	19	-1	0	1739455374017	f
12	30	8	1	0	1739455740543	f
12	2	8	1	0	1739455743293	f
12	31	0	-1	0	1739456206580	f
12	9	6	-1	0	1739456222703	f
12	9	30	1	0	1739456235556	f
12	9	11	1	0	1739456245559	f
12	30	6	-1	0	1739456720790	f
12	30	11	-1	0	1739456736753	f
12	3	29	0	0	1739456739097	f
12	0	31	0	0	1739457730950	f
12	5	31	-1	0	1739457754474	f
12	33	7	1	0	1739457782079	f
12	4	32	1	0	1739458352417	f
12	34	11	1	0	1739458360319	f
12	18	34	-1	0	1739459235129	f
12	20	2	-1	0	1739459751268	f
12	41	28	1	0	1739459993649	f
12	16	32	0	0	1739459995575	f
12	16	11	0	0	1739460005382	f
9	13	32	-1	0	1739460677580	f
9	13	34	-1	0	1739460684983	f
12	12	13	-1	0	1739461839197	f
12	12	34	-1	0	1739461844003	f
12	3	13	-1	0	1739461850982	f
12	39	19	-1	0	1739461854906	f
12	3	3	-1	0	1739461857102	f
12	39	23	-1	0	1739461860375	f
12	3	33	-1	0	1739461863230	f
12	16	14	-1	0	1739462547087	f
12	31	25	-1	0	1739462932029	f
12	14	14	0	0	1739462934749	f
12	53	23	-1	0	1739462939358	f
12	54	13	-1	0	1739462946044	f
12	54	1	-1	0	1739462949587	f
12	31	31	-1	0	1739462951274	f
12	31	33	-1	0	1739462959850	f
12	31	7	1	0	1739462975701	f
12	34	22	-1	0	1739463490144	f
12	17	20	-1	0	1739464096383	f
12	17	36	-1	0	1739464099511	f
12	49	22	-1	0	1739464105993	f
12	39	20	-1	0	1739464106047	f
12	49	25	-1	0	1739464119278	f
12	53	20	-1	0	1739464121129	f
12	21	22	-1	0	1739464132025	f
12	40	36	-1	0	1739464165607	f
12	29	35	1	0	1739464167819	f
12	39	30	1	0	1739464170863	f
12	15	36	-1	0	1739464175249	f
12	39	7	0	0	1739464176846	f
12	40	35	1	0	1739464177642	f
12	30	25	-1	0	1739464192006	f
12	57	14	1	0	1739464192302	f
12	0	37	0	0	1739464787552	f
12	27	35	0	0	1739465078919	f
12	27	21	1	0	1739465084062	f
12	20	18	-1	0	1739465098733	f
12	34	26	-1	0	1739465646357	f
12	31	18	-1	0	1739466276547	f
12	31	37	0	0	1739466280360	f
12	31	39	-1	0	1739466285493	f
12	3	23	-1	0	1739467872392	f
12	3	38	-1	0	1739467890156	f
12	60	26	-1	0	1739595941215	f
12	60	7	0	0	1739596241263	f
2	60	9	-1	0	1736732842649	f
2	60	41	1	0	1736732859741	f
2	60	49	-1	0	1736732875624	f
2	60	24	0	0	1736732883713	f
2	13	60	-1	0	1736734004732	f
2	13	66	-1	0	1736734008198	f
2	13	68	0	0	1736734014472	f
2	13	64	-1	0	1736734022253	f
2	13	65	-1	0	1736734031695	f
2	13	63	-1	0	1736734034046	f
2	13	62	-1	0	1736734035893	f
2	13	61	0	0	1736734040440	f
2	13	67	0	0	1736734051069	f
2	61	30	-1	0	1736734107433	f
2	61	2	0	0	1736734111976	f
2	61	6	1	0	1736734114354	f
2	61	38	-1	0	1736734116885	f
2	61	27	-1	0	1736734121499	f
2	61	60	0	0	1736734130554	f
2	61	1	1	0	1736734133218	f
2	61	66	-1	0	1736734136596	f
2	62	6	1	0	1736734137904	f
2	61	23	-1	0	1736734140031	f
2	61	64	-1	0	1736734143407	f
2	61	62	0	0	1736734156572	f
2	61	68	-1	0	1736734161648	f
2	61	9	-1	0	1736734163816	f
2	61	56	-1	0	1736734170952	f
2	61	61	1	0	1736734173102	f
2	61	58	0	0	1736734182246	f
2	61	5	0	0	1736734190785	f
2	61	59	-1	0	1736734194765	f
2	61	39	-1	0	1736734199469	f
2	61	63	-1	0	1736734203575	f
2	61	54	-1	0	1736734206660	f
2	61	10	-1	0	1736734209203	f
2	61	53	-1	0	1736734212239	f
2	61	40	-1	0	1736734215093	f
2	61	45	1	0	1736734271452	f
2	61	22	0	0	1736734282399	f
2	61	47	-1	0	1736734288168	f
2	61	28	-1	0	1736734296745	f
2	61	65	0	0	1736734308366	f
2	61	35	0	0	1736734324321	f
2	61	33	-1	0	1736734332553	f
2	61	46	-1	0	1736734337260	f
2	61	67	-1	0	1736734343201	f
2	61	36	0	0	1736734350093	f
2	61	31	-1	0	1736734368362	f
2	61	57	0	0	1736734371134	f
2	63	2	0	0	1736734372938	f
2	61	3	-1	0	1736734374537	f
2	61	41	-1	0	1736734387671	f
2	61	12	-1	0	1736734392334	f
2	64	2	1	0	1736734393768	f
2	63	30	-1	0	1736734395755	f
2	61	42	-1	0	1736734396630	f
2	64	1	1	0	1736734398471	f
2	61	25	-1	0	1736734401177	f
2	63	67	-1	0	1736734401455	f
2	63	66	-1	0	1736734405291	f
2	61	29	-1	0	1736734406179	f
2	63	1	1	0	1736734407717	f
2	61	21	-1	0	1736734411056	f
2	63	25	-1	0	1736734412637	f
2	61	15	-1	0	1736734413713	f
2	63	6	-1	0	1736734414737	f
2	64	30	0	0	1736734415256	f
2	63	9	-1	0	1736734417136	f
2	61	34	-1	0	1736734417263	f
2	61	43	-1	0	1736734419950	f
2	61	13	-1	0	1736734422536	f
2	64	66	-1	0	1736734423223	f
2	63	42	-1	0	1736734423307	f
2	61	48	1	0	1736734425267	f
2	63	12	-1	0	1736734426948	f
2	61	17	-1	0	1736734429800	f
2	63	68	-1	0	1736734431768	f
2	61	55	-1	0	1736734435407	f
2	63	3	-1	0	1736734437288	f
2	64	6	1	0	1736734438352	f
2	61	52	0	0	1736734439988	f
2	50	59	-1	0	1736734440571	f
2	61	11	-1	0	1736734443791	f
2	61	44	-1	0	1736734445925	f
2	63	56	0	0	1736734447847	f
2	50	68	-1	0	1736734448231	f
2	61	20	-1	0	1736734450783	f
2	63	57	0	0	1736734452437	f
2	50	67	-1	0	1736734453181	f
2	64	6	0	0	1736734455402	f
2	61	37	0	0	1736734457093	f
2	63	64	-1	0	1736734457157	f
2	50	66	-1	0	1736734458278	f
2	63	63	-1	0	1736734459557	f
2	61	51	-1	0	1736734460752	f
2	64	12	-1	0	1736734460812	f
2	50	63	-1	0	1736734461513	f
2	64	68	-1	0	1736734465293	f
2	61	32	0	0	1736734465314	f
2	63	59	0	0	1736734466797	f
2	50	52	-1	0	1736734467342	f
2	61	24	-1	0	1736734468744	f
2	64	65	0	0	1736734470412	f
2	63	60	0	0	1736734473067	f
2	50	47	-1	0	1736734473148	f
2	61	49	-1	0	1736734474054	f
2	64	39	-1	0	1736734474238	f
2	50	38	-1	0	1736734476023	f
2	64	9	-1	0	1736734477142	f
2	50	64	-1	0	1736734478698	f
2	63	58	-1	0	1736734480385	f
2	64	35	-1	0	1736734480974	f
2	61	50	-1	0	1736734481605	f
2	50	61	0	0	1736734482838	f
2	63	53	-1	0	1736734482917	f
2	64	59	-1	0	1736734484813	f
2	64	54	-1	0	1736734487683	f
2	64	17	-1	0	1736734489931	f
2	50	23	0	0	1736734493138	f
2	64	58	0	0	1736734493772	f
2	63	27	1	0	1736734496156	f
2	50	12	-1	0	1736734496224	f
2	64	64	-1	0	1736734498902	f
2	64	38	-1	0	1736734501132	f
2	50	58	0	0	1736734502198	f
2	64	34	-1	0	1736734505301	f
2	63	35	-1	0	1736734506517	f
2	50	33	-1	0	1736734507809	f
2	64	48	0	0	1736734508500	f
2	63	54	-1	0	1736734511123	f
2	50	35	-1	0	1736734512910	f
2	64	67	0	0	1736734512979	f
2	50	25	-1	0	1736734515639	f
2	64	27	-1	0	1736734519372	f
2	64	21	-1	0	1736734522877	f
2	63	48	0	0	1736734523127	f
2	77	1	1	0	1736744251396	f
5	0	3	0	0	1736789804780	f
5	0	15	-1	0	1736794792291	f
5	9	19	0	0	1736796245486	f
5	8	20	-1	0	1736796439132	f
5	14	18	-1	0	1736803832877	f
5	17	10	1	0	1736809745526	f
5	17	22	-1	0	1736809755395	f
5	17	3	1	0	1736809759920	f
2	78	6	-1	0	1736826753912	f
2	78	56	-1	0	1736827671097	f
2	78	29	0	0	1736827693656	f
2	78	39	0	0	1736827715800	f
2	78	22	0	0	1736827733101	f
2	78	47	-1	0	1736827755812	f
2	79	3	-1	0	1736879846019	f
2	79	20	-1	0	1736879858719	f
2	81	1	1	0	1736879876179	f
2	79	59	-1	0	1736879883898	f
2	81	6	1	0	1736879891278	f
2	82	39	-1	0	1736880598882	f
2	82	48	-1	0	1736880942617	f
2	86	65	-1	0	1736882317495	f
2	86	70	-1	0	1736882329386	f
2	87	55	-1	0	1736916292356	f
2	87	32	0	0	1736917156922	f
2	88	39	0	0	1736994681538	f
2	89	72	1	0	1737002015165	f
2	89	42	-1	0	1737002585230	f
2	89	64	-1	0	1737002598980	f
2	90	28	-1	0	1737041560321	f
2	90	34	-1	0	1737041576953	f
2	90	51	1	0	1737041584814	f
2	13	73	-1	0	1737062889085	f
2	13	70	0	0	1737062905509	f
2	92	64	-1	0	1737467553842	f
2	92	68	-1	0	1737467577405	f
2	92	31	-1	0	1737467588479	f
11	12	8	-1	0	1739316197460	f
9	3	0	-1	0	1739371790539	f
12	0	11	-1	0	1739382106953	f
12	2	4	-1	0	1739454349384	f
12	14	0	0	0	1739454843930	f
9	11	31	0	0	1739455389412	f
12	8	28	1	0	1739455772822	f
12	10	28	1	0	1739455784769	f
12	8	19	-1	0	1739455785682	f
12	26	6	0	0	1739456234365	f
12	3	30	0	0	1739456747947	f
12	3	6	-1	0	1739456758975	f
12	33	29	-1	0	1739457739235	f
12	33	23	-1	0	1739457761672	f
12	2	31	-1	0	1739457763897	f
12	34	17	1	0	1739458413045	f
12	34	28	1	0	1739458428803	f
12	34	23	-1	0	1739458444835	f
12	34	1	0	0	1739459413396	f
12	20	3	1	0	1739459771313	f
12	20	31	-1	0	1739459785657	f
12	39	28	-1	0	1739459796817	f
12	14	11	1	0	1739459995272	f
12	40	27	1	0	1739460001662	f
12	17	8	0	0	1739460004816	f
12	40	8	1	0	1739460008789	f
12	27	13	-1	0	1739460030931	f
12	45	34	-1	0	1739460701318	f
12	39	27	1	0	1739461840232	f
12	47	4	-1	0	1739461841272	f
12	3	2	-1	0	1739461854209	f
12	39	1	-1	0	1739461865189	f
12	3	1	-1	0	1739461869405	f
12	3	34	-1	0	1739461873249	f
12	48	31	-1	0	1739461875400	f
12	16	25	-1	0	1739462551963	f
12	31	13	1	0	1739462936302	f
12	54	19	-1	0	1739462939491	f
12	11	34	-1	0	1739463533477	f
12	41	36	1	0	1739464117319	f
12	39	6	-1	0	1739464121885	f
12	39	36	-1	0	1739464126346	f
12	49	14	-1	0	1739464126412	f
12	29	1	-1	0	1739464150904	f
12	21	35	-1	0	1739464150996	f
12	29	14	-1	0	1739464160278	f
12	15	2	-1	0	1739464168421	f
12	29	36	-1	0	1739464171447	f
12	35	27	0	0	1739464174253	f
12	57	23	-1	0	1739464177586	f
12	35	36	-1	0	1739464179709	f
12	35	23	-1	0	1739464194289	f
12	57	36	-1	0	1739464195132	f
12	57	20	-1	0	1739464199659	f
12	35	20	-1	0	1739464213869	f
12	40	25	-1	0	1739464229401	f
12	29	13	-1	0	1739464231382	f
12	15	14	0	0	1739464232087	f
12	35	22	-1	0	1739464235891	f
12	53	19	-1	0	1739464238026	f
12	16	37	-1	0	1739464811601	f
12	30	37	-1	0	1739465121543	f
12	12	26	-1	0	1739465718391	f
12	30	26	0	0	1739466289928	f
12	3	26	-1	0	1739467879652	f
12	60	41	0	0	1739595960905	f
12	60	30	1	0	1739596281297	f
12	60	32	1	0	1739596295734	f
2	61	69	-1	0	1736734528773	f
2	63	62	0	0	1736734538200	f
2	63	55	-1	0	1736734541637	f
2	63	29	-1	0	1736734554308	f
2	65	2	0	0	1736734566295	f
2	63	34	-1	0	1736734566525	f
2	61	70	-1	0	1736734572304	f
2	65	67	-1	0	1736734572455	f
2	63	69	-1	0	1736734574175	f
2	64	60	-1	0	1736734574790	f
2	63	43	-1	0	1736734576988	f
2	63	33	-1	0	1736734581078	f
2	64	28	-1	0	1736734581476	f
2	63	52	0	0	1736734585390	f
2	64	70	-1	0	1736734586916	f
2	65	30	1	0	1736734586954	f
2	64	69	0	0	1736734589991	f
2	63	11	0	0	1736734592798	f
2	64	55	-1	0	1736734593316	f
2	65	69	1	0	1736734594805	f
2	63	70	-1	0	1736734596267	f
2	64	42	-1	0	1736734596527	f
2	65	1	0	0	1736734600196	f
2	63	65	-1	0	1736734600667	f
2	65	54	-1	0	1736734604154	f
2	63	31	-1	0	1736734605548	f
2	63	13	-1	0	1736734607897	f
2	65	70	-1	0	1736734610485	f
2	63	46	-1	0	1736734612128	f
2	65	6	0	0	1736734614215	f
2	63	38	1	0	1736734615709	f
2	63	23	-1	0	1736734620419	f
2	65	60	-1	0	1736734620576	f
2	64	56	-1	0	1736734621380	f
2	64	63	-1	0	1736734624378	f
2	65	66	-1	0	1736734625855	f
2	64	3	-1	0	1736734626310	f
2	64	32	-1	0	1736734630461	f
2	65	27	-1	0	1736734630797	f
2	64	5	-1	0	1736734632366	f
2	64	61	-1	0	1736734653478	f
2	64	40	-1	0	1736734658276	f
2	64	23	-1	0	1736734665671	f
2	64	41	0	0	1736734675557	f
2	64	45	-1	0	1736734681326	f
2	65	25	-1	0	1736734681832	f
2	65	3	-1	0	1736734685313	f
2	64	37	-1	0	1736734687728	f
2	65	38	-1	0	1736734689090	f
2	64	47	-1	0	1736734694765	f
2	65	33	0	0	1736734696235	f
2	65	37	-1	0	1736734700712	f
2	64	33	-1	0	1736734704040	f
2	65	51	0	0	1736734706974	f
2	64	22	0	0	1736734710707	f
2	65	53	-1	0	1736734712618	f
2	64	53	-1	0	1736734715064	f
2	65	55	0	0	1736734723950	f
2	65	12	-1	0	1736734727968	f
2	65	58	0	0	1736734737649	f
2	64	13	-1	0	1736734742562	f
2	65	46	0	0	1736734746248	f
2	65	64	-1	0	1736734754238	f
2	65	28	-1	0	1736734758574	f
2	64	43	-1	0	1736734764003	f
2	50	15	-1	0	1736734766915	f
2	64	31	-1	0	1736734770333	f
2	50	69	-1	0	1736734771589	f
2	64	20	-1	0	1736734773512	f
2	50	13	0	0	1736734775498	f
2	64	50	-1	0	1736734775919	f
2	50	27	0	0	1736734781018	f
2	64	62	0	0	1736734781239	f
2	64	25	-1	0	1736734783132	f
2	50	3	-1	0	1736734784970	f
2	64	46	-1	0	1736734786332	f
2	64	36	-1	0	1736734789212	f
2	65	56	-1	0	1736734791549	f
2	64	44	-1	0	1736734792189	f
2	65	20	-1	0	1736734798514	f
2	64	52	0	0	1736734798987	f
2	64	51	-1	0	1736734801239	f
2	65	31	-1	0	1736734804810	f
2	64	11	1	0	1736734805550	f
2	65	68	0	0	1736734813535	f
2	62	2	1	0	1736734819245	f
2	64	15	-1	0	1736734822209	f
2	64	10	-1	0	1736734824087	f
2	62	33	-1	0	1736734824150	f
2	62	35	-1	0	1736734828455	f
2	64	29	0	0	1736734829838	f
2	62	1	1	0	1736734830192	f
2	64	57	0	0	1736734833236	f
2	62	59	-1	0	1736734836234	f
2	64	24	1	0	1736734837391	f
2	64	49	-1	0	1736734841857	f
2	62	47	-1	0	1736734857128	f
2	62	66	-1	0	1736734860201	f
2	62	70	1	0	1736734863575	f
2	62	37	0	0	1736734868601	f
2	62	30	1	0	1736734873204	f
2	62	61	-1	0	1736734882316	f
2	62	63	-1	0	1736734885490	f
2	62	45	-1	0	1736734894506	f
2	62	53	-1	0	1736734901675	f
2	62	68	-1	0	1736734913035	f
2	62	69	1	0	1736734922048	f
2	66	6	1	0	1736735101047	f
2	66	1	1	0	1736735102813	f
2	66	30	0	0	1736735119249	f
2	66	64	-1	0	1736735122982	f
2	66	70	-1	0	1736735125858	f
2	66	33	-1	0	1736735131640	f
2	66	63	-1	0	1736735133861	f
2	66	2	-1	0	1736735142509	f
2	66	59	-1	0	1736735149607	f
2	66	42	-1	0	1736735153886	f
2	66	38	1	0	1736735156608	f
2	66	20	-1	0	1736735164354	f
2	66	39	-1	0	1736735169426	f
2	66	67	-1	0	1736735174960	f
2	66	66	-1	0	1736735178328	f
2	66	69	-1	0	1736735181973	f
2	66	43	-1	0	1736735184458	f
2	66	15	-1	0	1736735323764	f
2	66	47	-1	0	1736735327737	f
2	66	34	-1	0	1736735331557	f
2	66	40	-1	0	1736735334123	f
2	66	61	-1	0	1736735336354	f
2	66	56	-1	0	1736735339290	f
2	66	48	-1	0	1736735376485	f
2	66	68	-1	0	1736735380633	f
2	66	54	-1	0	1736735385247	f
2	66	37	-1	0	1736735450247	f
2	66	55	-1	0	1736735459381	f
2	66	65	-1	0	1736735462325	f
2	66	13	-1	0	1736735465109	f
2	66	10	-1	0	1736735468684	f
2	77	2	1	0	1736744257200	f
5	0	4	0	0	1736789850858	f
5	0	14	-1	0	1736794796024	f
5	9	20	-1	0	1736797017857	f
5	14	20	-1	0	1736803850477	f
5	14	9	1	0	1736803866150	f
5	17	5	-1	0	1736809748007	f
5	17	12	1	0	1736809751367	f
5	17	2	1	0	1736809757541	f
5	17	11	1	0	1736809766393	f
5	17	14	-1	0	1736809804073	f
2	78	48	0	0	1736827214740	f
2	78	60	-1	0	1736827230976	f
2	78	65	-1	0	1736827677162	f
2	78	46	-1	0	1736827708970	f
2	78	25	-1	0	1736827718733	f
2	78	51	0	0	1736827752047	f
2	78	3	-1	0	1736827760445	f
2	78	50	0	0	1736827775420	f
2	79	34	-1	0	1736879865243	f
2	82	46	1	0	1736880653412	f
2	82	20	-1	0	1736880947043	f
2	86	42	-1	0	1736882360116	f
2	86	59	-1	0	1736882390229	f
2	86	32	0	0	1736882414850	f
2	87	73	0	0	1736916298469	f
2	87	70	1	0	1736916306691	f
2	87	36	-1	0	1736917161405	f
2	88	34	-1	0	1736994695369	f
2	89	39	-1	0	1737002032595	f
2	89	15	-1	0	1737002589051	f
2	89	51	1	0	1737002602947	f
2	90	45	-1	0	1737041569197	f
2	13	72	0	0	1737062909827	f
2	92	60	-1	0	1737467560840	f
2	92	20	-1	0	1737467563093	f
2	92	22	-1	0	1737467574153	f
2	92	29	-1	0	1737467582328	f
11	12	9	-1	0	1739316275290	f
9	3	1	-1	0	1739371792551	f
12	1	8	0	0	1739382153182	f
9	10	29	0	0	1739454374672	f
12	26	4	-1	0	1739454852991	f
12	26	0	-1	0	1739454865165	f
12	27	17	-1	0	1739454891768	f
9	11	29	0	0	1739455395543	f
12	30	0	1	0	1739455776765	f
12	8	29	1	0	1739455780401	f
12	26	30	0	0	1739456260855	f
12	3	11	0	0	1739456753522	f
12	33	28	1	0	1739457802969	f
9	12	37	-1	0	1739457820994	f
9	12	31	0	0	1739457836653	f
12	34	6	-1	0	1739458478027	f
12	34	13	1	0	1739459426522	f
12	13	19	-1	0	1739459776345	f
12	20	33	-1	0	1739459777945	f
12	27	1	-1	0	1739460040966	f
9	13	31	-1	0	1739460714548	f
9	13	38	-1	0	1739460719117	f
9	13	37	-1	0	1739460724685	f
12	45	31	-1	0	1739460728486	f
12	39	3	-1	0	1739461868156	f
12	2	14	-1	0	1739462574247	f
12	54	7	0	0	1739462975962	f
12	11	13	-1	0	1739463538051	f
12	15	35	1	0	1739464118635	f
12	36	36	1	0	1739464127871	f
12	49	36	-1	0	1739464128595	f
12	15	27	-1	0	1739464129423	f
12	39	11	1	0	1739464139605	f
12	21	25	-1	0	1739464146121	f
12	34	37	0	0	1739464835484	f
12	53	18	-1	0	1739465154049	f
12	53	32	1	0	1739465236103	f
12	0	38	0	0	1739465889801	f
12	41	18	-1	0	1739465944293	f
12	5	26	-1	0	1739465949668	f
12	49	21	1	0	1739465960613	f
12	31	26	-1	0	1739466290016	f
12	31	38	-1	0	1739466293585	f
12	3	39	-1	0	1739467885479	f
12	60	3	-1	0	1739595964009	f
12	60	8	1	0	1739596286725	f
12	60	17	-1	0	1739596300565	f
2	66	49	0	0	1736735475701	f
2	66	21	-1	0	1736735479135	f
2	66	60	-1	0	1736735504088	f
2	66	22	-1	0	1736735511293	f
2	66	5	0	0	1736735515564	f
2	66	25	-1	0	1736735518966	f
2	66	12	-1	0	1736735521056	f
2	66	58	-1	0	1736735529718	f
2	66	9	-1	0	1736735531366	f
2	66	31	-1	0	1736735537324	f
2	66	17	-1	0	1736735543121	f
2	66	35	-1	0	1736735558930	f
2	66	46	-1	0	1736735563442	f
2	66	44	-1	0	1736735565494	f
2	66	32	-1	0	1736735569796	f
2	66	3	-1	0	1736735615450	f
2	66	29	-1	0	1736735621484	f
2	67	1	0	0	1736735626145	f
2	67	6	0	0	1736735627818	f
2	66	27	0	0	1736735628235	f
2	66	45	-1	0	1736735634550	f
2	66	53	-1	0	1736735640284	f
2	66	28	-1	0	1736735644094	f
2	66	11	-1	0	1736735646496	f
2	67	30	1	0	1736735647502	f
2	66	51	-1	0	1736735649167	f
2	67	63	-1	0	1736735650448	f
2	66	36	-1	0	1736735653216	f
2	67	2	1	0	1736735653388	f
2	66	23	-1	0	1736735660612	f
2	67	70	0	0	1736735660891	f
2	67	52	0	0	1736735664174	f
2	66	62	-1	0	1736735664440	f
2	67	34	-1	0	1736735668043	f
2	66	50	-1	0	1736735669321	f
2	68	2	0	0	1736735670848	f
2	67	66	-1	0	1736735671495	f
2	67	53	-1	0	1736735673645	f
2	68	6	1	0	1736735674763	f
2	67	21	-1	0	1736735678278	f
2	66	24	-1	0	1736735680023	f
2	66	41	0	0	1736735686377	f
2	66	52	0	0	1736735690148	f
2	66	57	0	0	1736735694142	f
2	68	31	0	0	1736735703155	f
2	69	6	0	0	1736735704405	f
2	68	30	-1	0	1736735713396	f
2	68	1	1	0	1736735715486	f
2	68	63	-1	0	1736735719423	f
2	68	9	-1	0	1736735723417	f
2	68	12	-1	0	1736735731723	f
2	68	3	-1	0	1736735735041	f
2	68	56	0	0	1736735743492	f
2	68	59	0	0	1736735754053	f
2	68	69	-1	0	1736735762366	f
2	68	64	-1	0	1736735768582	f
2	69	66	0	0	1736735770936	f
2	68	66	-1	0	1736735771816	f
2	69	59	-1	0	1736735774716	f
2	69	70	0	0	1736735778179	f
2	69	1	0	0	1736735779469	f
2	68	33	1	0	1736735780197	f
2	69	2	0	0	1736735781230	f
2	69	24	0	0	1736735783019	f
2	69	30	0	0	1736735785183	f
2	69	34	-1	0	1736735789334	f
2	68	47	0	0	1736735789854	f
2	69	69	-1	0	1736735791693	f
2	68	27	0	0	1736735795576	f
2	69	64	-1	0	1736735796022	f
2	69	22	1	0	1736735798888	f
2	68	34	0	0	1736735802758	f
2	69	58	1	0	1736735806095	f
2	69	65	-1	0	1736735809587	f
2	68	60	-1	0	1736735809690	f
2	69	33	-1	0	1736735811413	f
2	69	37	-1	0	1736735816264	f
2	68	68	-1	0	1736735816577	f
2	69	5	-1	0	1736735818023	f
2	68	42	-1	0	1736735821026	f
2	69	10	-1	0	1736735821044	f
2	68	38	1	0	1736735824337	f
2	69	60	-1	0	1736735825972	f
2	68	11	-1	0	1736735826930	f
2	69	21	-1	0	1736735827585	f
2	69	68	-1	0	1736735831708	f
2	68	21	-1	0	1736735833377	f
2	69	39	-1	0	1736735836654	f
2	68	67	1	0	1736735840499	f
2	69	47	-1	0	1736735842725	f
2	69	54	-1	0	1736735845324	f
2	68	20	-1	0	1736735845744	f
2	69	67	-1	0	1736735848441	f
2	68	51	0	0	1736735849478	f
2	69	15	-1	0	1736735850398	f
2	68	37	-1	0	1736735853883	f
2	69	55	-1	0	1736735854941	f
2	69	27	-1	0	1736735857456	f
2	69	48	0	0	1736735859796	f
2	68	70	1	0	1736735861370	f
2	69	35	-1	0	1736735863365	f
2	69	38	-1	0	1736735865175	f
2	69	9	-1	0	1736735866477	f
2	69	62	1	0	1736735871657	f
2	68	55	0	0	1736735876675	f
2	69	36	-1	0	1736735877587	f
2	69	23	-1	0	1736735880377	f
2	68	13	1	0	1736735881819	f
2	69	31	-1	0	1736735883999	f
2	69	44	-1	0	1736735888033	f
2	68	28	0	0	1736735889122	f
2	69	53	-1	0	1736735890218	f
2	69	63	-1	0	1736735893032	f
2	68	25	-1	0	1736735893076	f
2	69	61	-1	0	1736735895232	f
2	69	42	-1	0	1736735899875	f
2	69	13	-1	0	1736735901526	f
2	68	35	-1	0	1736735902040	f
2	68	62	0	0	1736735907797	f
2	69	32	-1	0	1736735909295	f
2	69	3	-1	0	1736735911131	f
2	68	10	-1	0	1736735911472	f
2	69	17	-1	0	1736735912843	f
2	68	15	-1	0	1736735914914	f
2	69	51	-1	0	1736735915208	f
2	69	45	-1	0	1736735919836	f
2	68	53	0	0	1736735920611	f
2	69	20	-1	0	1736735923108	f
2	69	12	-1	0	1736735925499	f
2	68	45	-1	0	1736735930332	f
2	69	56	-1	0	1736735932121	f
2	69	50	0	0	1736735936006	f
2	69	40	-1	0	1736735938011	f
2	69	57	-1	0	1736735941029	f
2	68	39	-1	0	1736735941100	f
2	69	46	-1	0	1736735943849	f
2	69	41	0	0	1736735948469	f
2	69	25	-1	0	1736735952660	f
2	68	65	-1	0	1736735962669	f
2	68	40	1	0	1736735967084	f
2	68	61	0	0	1736735977540	f
2	68	24	-1	0	1736735990529	f
2	68	22	-1	0	1736735996229	f
2	68	48	-1	0	1736736003482	f
2	68	5	-1	0	1736736015951	f
2	77	55	-1	0	1736744295099	f
5	0	5	0	0	1736789904628	f
5	5	1	1	0	1736794871154	f
5	5	6	0	0	1736794905169	f
5	10	9	0	0	1736797077204	f
5	10	5	-1	0	1736797087361	f
5	10	8	-1	0	1736797091848	f
5	10	17	-1	0	1736797102413	f
5	10	14	-1	0	1736797124857	f
5	10	7	-1	0	1736797133604	f
5	10	12	0	0	1736797183918	f
5	10	11	0	0	1736797186806	f
5	14	17	-1	0	1736803856554	f
5	14	13	1	0	1736803879618	f
5	14	5	1	0	1736803911322	f
5	14	15	-1	0	1736803926819	f
5	14	16	1	0	1736803937247	f
5	14	6	1	0	1736803946723	f
5	14	2	1	0	1736803949900	f
5	17	6	-1	0	1736809792308	f
5	17	7	-1	0	1736809796682	f
5	17	21	-1	0	1736809811710	f
2	78	72	0	0	1736827220716	f
2	78	33	-1	0	1736827244101	f
2	78	64	-1	0	1736827682045	f
2	79	68	-1	0	1736879871226	f
2	81	2	0	0	1736879881456	f
2	79	21	-1	0	1736879889879	f
2	82	10	-1	0	1736880661640	f
2	82	22	1	0	1736880952827	f
2	86	41	1	0	1736882372633	f
2	86	40	-1	0	1736882396342	f
2	87	27	-1	0	1736916366995	f
2	87	65	-1	0	1736917166775	f
2	88	27	-1	0	1736994703869	f
2	89	2	-1	0	1737002036899	f
2	90	1	1	0	1737041230373	f
2	90	20	-1	0	1737041596277	f
6	0	0	0	0	1737084575710	f
2	92	49	1	0	1737467607551	f
2	92	71	0	0	1737467618880	f
2	92	45	-1	0	1737467636818	f
2	92	57	-1	0	1737467641798	f
2	92	61	-1	0	1737467652960	f
9	4	2	-1	0	1739378620474	f
12	0	12	-1	0	1739382186106	f
9	10	30	0	0	1739454378225	f
12	27	4	0	0	1739454897495	f
9	11	30	0	0	1739455397455	f
12	10	27	1	0	1739455781164	f
12	8	27	1	0	1739455789778	f
12	8	8	-1	0	1739455811603	f
12	10	29	1	0	1739455823394	f
12	2	11	1	0	1739456284279	f
12	5	11	1	0	1739456304677	f
12	3	8	0	0	1739456765235	f
9	8	34	1	0	1739456787771	f
9	12	36	-1	0	1739457808999	f
9	12	32	-1	0	1739457814545	f
12	33	27	1	0	1739457827501	f
9	12	38	-1	0	1739457829563	f
9	12	30	0	0	1739457832307	f
12	0	33	0	0	1739458495034	f
12	18	13	-1	0	1739459437335	f
12	21	13	-1	0	1739459463348	f
12	5	13	-1	0	1739459466342	f
12	13	6	-1	0	1739459789953	f
12	38	31	1	0	1739459791010	f
12	10	31	-1	0	1739459795163	f
12	44	23	-1	0	1739460044149	f
12	27	30	0	0	1739460051022	f
12	44	17	0	0	1739460070308	f
12	27	31	-1	0	1739460081901	f
12	27	28	1	0	1739460089903	f
12	27	11	1	0	1739460121409	f
12	44	13	-1	0	1739460137309	f
9	13	35	-1	0	1739460721333	f
12	39	2	-1	0	1739461870871	f
12	49	0	-1	0	1739461885620	f
12	2	25	-1	0	1739462578014	f
12	54	17	0	0	1739462982566	f
12	2	22	-1	0	1739463598363	f
12	30	14	-1	0	1739464145555	f
12	39	4	0	0	1739464148414	f
12	15	11	1	0	1739464151637	f
12	53	29	0	0	1739464151953	f
12	21	36	-1	0	1739464154767	f
12	20	14	-1	0	1739464157921	f
12	35	4	0	0	1739464158008	f
12	57	3	1	0	1739464172713	f
12	30	36	0	0	1739464172991	f
12	30	35	-1	0	1739464181766	f
12	56	29	0	0	1739464182841	f
12	15	33	-1	0	1739464186594	f
12	20	35	1	0	1739464190653	f
12	15	19	-1	0	1739464191413	f
12	21	21	-1	0	1739464861483	f
12	30	21	-1	0	1739465156186	f
12	15	38	-1	0	1739465930480	f
12	15	26	-1	0	1739465938177	f
12	41	26	-1	0	1739465938731	f
12	22	20	-1	0	1739465940594	f
12	5	38	-1	0	1739465952105	f
12	22	25	-1	0	1739465960908	f
12	22	14	-1	0	1739465967584	f
12	59	0	-1	0	1739465969716	f
12	59	32	0	0	1739465985280	f
12	59	22	-1	0	1739466006679	f
12	59	18	-1	0	1739466011495	f
12	59	34	1	0	1739466026443	f
12	59	1	-1	0	1739466029077	f
12	15	39	-1	0	1739466030030	f
12	59	20	-1	0	1739466031969	f
12	31	21	-1	0	1739466301196	f
12	27	39	-1	0	1739466338136	f
12	0	40	0	0	1739468072686	f
12	60	0	1	0	1739595979362	f
12	60	18	0	0	1739596000790	f
12	60	11	1	0	1739596330075	f
2	69	43	0	0	1736735950490	f
2	68	17	-1	0	1736735957573	f
2	69	49	-1	0	1736735961753	f
2	68	54	-1	0	1736735972243	f
2	69	11	-1	0	1736735977487	f
2	68	23	-1	0	1736735999263	f
2	68	50	-1	0	1736736013136	f
2	68	36	-1	0	1736736029720	f
2	68	41	0	0	1736736048461	f
2	77	58	-1	0	1736744304247	f
5	0	6	0	0	1736791252099	f
5	5	13	-1	0	1736794881251	f
5	5	10	0	0	1736794884823	f
5	10	18	-1	0	1736797084878	f
5	10	13	-1	0	1736797097611	f
5	10	4	-1	0	1736797107644	f
5	10	6	-1	0	1736797162275	f
5	10	20	-1	0	1736797175269	f
5	10	10	0	0	1736797180395	f
5	10	3	-1	0	1736797182390	f
5	14	19	1	0	1736803862843	f
5	14	7	1	0	1736803893027	f
5	14	4	0	0	1736803906487	f
5	14	14	-1	0	1736803922017	f
5	14	1	-1	0	1736803932872	f
5	17	8	-1	0	1736809821847	f
5	17	20	-1	0	1736809824492	f
5	17	18	-1	0	1736809828973	f
2	78	66	-1	0	1736827227105	f
2	78	13	-1	0	1736827266222	f
2	78	49	0	0	1736827700476	f
2	79	38	-1	0	1736879897543	f
2	79	9	1	0	1736879905014	f
2	82	34	0	0	1736880679615	f
2	82	43	1	0	1736880696490	f
2	82	66	0	0	1736880718752	f
2	82	41	-1	0	1736880994452	f
2	86	27	1	0	1736882380717	f
2	86	37	-1	0	1736882385174	f
2	86	68	-1	0	1736882393604	f
2	87	20	0	0	1736916373449	f
2	87	29	-1	0	1736916401492	f
2	87	58	1	0	1736917229073	f
2	87	61	-1	0	1736917244873	f
2	88	42	-1	0	1736994713392	f
2	88	31	-1	0	1736994732802	f
2	89	1	0	0	1737002045401	f
2	90	2	1	0	1737041234908	f
2	90	6	1	0	1737041240352	f
2	90	46	0	0	1737041636696	f
2	90	48	-1	0	1737041646761	f
6	0	1	-1	0	1737096160075	f
2	92	9	-1	0	1737467610464	f
2	92	48	1	0	1737467632016	f
2	92	52	-1	0	1737467639310	f
2	92	24	0	0	1737467664706	f
9	5	1	-1	0	1739378649942	f
9	4	1	-1	0	1739378704340	f
12	0	13	-1	0	1739382202235	f
12	3	4	-1	0	1739454392223	f
12	3	0	1	0	1739454402014	f
12	27	0	-1	0	1739454902366	f
12	20	27	-1	0	1739455517757	f
12	8	23	-1	0	1739455816146	f
12	5	29	1	0	1739456284557	f
12	5	6	-1	0	1739456308200	f
9	8	33	1	0	1739456794429	f
12	33	4	1	0	1739457837212	f
9	12	29	0	0	1739457840146	f
12	34	7	0	0	1739458497159	f
12	34	19	-1	0	1739458514954	f
12	21	30	1	0	1739459445048	f
12	22	34	0	0	1739459790062	f
12	38	17	-1	0	1739459811907	f
12	27	32	0	0	1739460075874	f
9	13	30	0	0	1739460728817	f
12	49	4	-1	0	1739461881540	f
12	49	34	0	0	1739461889297	f
12	49	3	-1	0	1739461892336	f
12	49	27	-1	0	1739461898778	f
12	50	0	1	0	1739461906770	f
12	49	17	-1	0	1739461924889	f
12	49	2	-1	0	1739461942776	f
9	14	34	-1	0	1739462748009	f
12	31	32	1	0	1739462985534	f
12	54	31	-1	0	1739462998304	f
12	54	33	-1	0	1739463020243	f
12	54	8	1	0	1739463061845	f
12	2	20	-1	0	1739463643070	f
12	56	34	-1	0	1739464193920	f
12	29	7	0	0	1739464198876	f
12	56	19	-1	0	1739464199159	f
12	40	14	0	0	1739464199943	f
12	58	29	-1	0	1739464201205	f
12	15	3	1	0	1739464203995	f
12	57	25	-1	0	1739464204823	f
12	53	22	-1	0	1739464206432	f
12	10	36	-1	0	1739464228965	f
12	17	37	0	0	1739464863023	f
12	53	31	-1	0	1739465172200	f
12	53	2	-1	0	1739465187004	f
12	41	38	-1	0	1739465933279	f
12	22	38	-1	0	1739465943191	f
12	22	26	-1	0	1739465950589	f
12	59	27	-1	0	1739465967813	f
12	35	38	-1	0	1739466326645	f
12	27	38	0	0	1739466329159	f
12	5	40	-1	0	1739468106962	f
12	60	6	-1	0	1739595984877	f
12	60	1	-1	0	1739595993792	f
12	60	40	-1	0	1739596003122	f
12	60	28	1	0	1739596338719	f
12	60	44	-1	0	1739596375576	f
12	60	45	-1	0	1739596514353	f
2	68	58	0	0	1736735953305	f
2	69	29	-1	0	1736735958240	f
2	69	28	-1	0	1736735970805	f
2	69	52	1	0	1736735975303	f
2	68	46	-1	0	1736735983646	f
2	68	43	1	0	1736736008033	f
2	68	29	0	0	1736736023933	f
2	68	52	-1	0	1736736053145	f
2	68	44	0	0	1736736068217	f
2	68	32	-1	0	1736736073871	f
2	68	49	-1	0	1736736078230	f
2	68	57	0	0	1736736082048	f
2	0	64	-1	0	1736736231796	f
2	0	69	-1	0	1736736237007	f
2	0	66	-1	0	1736736239194	f
2	0	67	-1	0	1736736241580	f
2	0	68	-1	0	1736736244729	f
2	0	56	0	0	1736736254926	f
2	68	71	-1	0	1736736258547	f
2	0	70	-1	0	1736736260278	f
2	0	62	-1	0	1736736265478	f
2	0	71	-1	0	1736736281383	f
2	0	58	-1	0	1736736285475	f
2	0	55	-1	0	1736736291382	f
2	0	60	-1	0	1736736298940	f
2	0	54	-1	0	1736736302066	f
2	0	65	-1	0	1736736304521	f
2	0	53	-1	0	1736736307991	f
2	0	59	-1	0	1736736311101	f
2	0	72	-1	0	1736736347316	f
2	0	61	0	0	1736736363080	f
2	0	57	0	0	1736736364810	f
2	70	54	-1	0	1736736427830	f
2	70	6	0	0	1736736433132	f
2	70	30	0	0	1736736441687	f
2	70	2	-1	0	1736736445530	f
2	71	1	1	0	1736736464002	f
2	70	71	0	0	1736736464232	f
2	71	6	1	0	1736736465816	f
2	70	1	0	0	1736736468770	f
2	71	13	0	0	1736736468818	f
2	71	40	-1	0	1736736470572	f
2	71	71	-1	0	1736736474233	f
2	71	2	1	0	1736736477438	f
2	71	38	-1	0	1736736479013	f
2	71	63	-1	0	1736736480692	f
2	71	12	-1	0	1736736484082	f
2	71	30	1	0	1736736485973	f
2	71	53	-1	0	1736736487773	f
2	71	65	-1	0	1736736490273	f
2	72	2	-1	0	1736736532730	f
2	72	1	1	0	1736736535268	f
2	72	30	0	0	1736736542283	f
2	72	38	-1	0	1736736545085	f
2	72	6	-1	0	1736736547450	f
2	72	72	-1	0	1736736556048	f
2	72	60	-1	0	1736736567957	f
2	72	71	-1	0	1736736581778	f
2	72	58	-1	0	1736736587847	f
2	72	10	-1	0	1736736595018	f
2	72	34	-1	0	1736736601234	f
2	72	13	-1	0	1736736605433	f
2	72	64	-1	0	1736736608498	f
2	72	5	-1	0	1736736617688	f
2	73	34	-1	0	1736736989043	f
2	73	64	-1	0	1736736991194	f
2	73	30	1	0	1736736996366	f
2	73	1	1	0	1736736998646	f
2	73	71	1	0	1736737009269	f
2	73	6	-1	0	1736737012049	f
2	73	2	-1	0	1736737015918	f
2	73	69	-1	0	1736737019197	f
2	73	23	-1	0	1736737022410	f
2	73	33	1	0	1736737027622	f
2	73	17	-1	0	1736737030295	f
2	73	12	-1	0	1736737032947	f
2	73	37	-1	0	1736737036928	f
2	73	68	-1	0	1736737040021	f
2	70	34	0	0	1736737040253	f
2	73	38	-1	0	1736737041660	f
2	70	3	-1	0	1736737043614	f
2	73	51	-1	0	1736737043630	f
2	73	72	-1	0	1736737047261	f
2	73	70	-1	0	1736737050834	f
2	70	72	-1	0	1736737054151	f
2	73	55	-1	0	1736737056631	f
2	73	3	-1	0	1736737058664	f
2	70	42	-1	0	1736737059450	f
2	70	64	-1	0	1736737063830	f
2	70	17	-1	0	1736737066526	f
2	73	39	0	0	1736737079696	f
2	71	59	-1	0	1736737656485	f
2	71	39	-1	0	1736737662861	f
2	71	29	-1	0	1736737666625	f
2	71	60	0	0	1736737676869	f
2	71	72	0	0	1736737679322	f
2	71	64	-1	0	1736737681222	f
2	71	5	-1	0	1736737682842	f
2	71	33	0	0	1736737720762	f
2	71	15	-1	0	1736737722902	f
2	71	25	-1	0	1736737724820	f
2	71	51	1	0	1736737728652	f
2	71	56	-1	0	1736737732700	f
2	72	69	0	0	1736738095956	f
2	72	55	-1	0	1736738100005	f
2	72	67	-1	0	1736738103864	f
2	72	39	-1	0	1736738108704	f
2	72	52	0	0	1736738115797	f
2	72	21	-1	0	1736738120131	f
2	72	11	-1	0	1736738123651	f
2	72	15	-1	0	1736738127563	f
2	72	68	0	0	1736738136858	f
2	72	40	-1	0	1736738140830	f
2	72	33	-1	0	1736738145663	f
2	72	45	-1	0	1736738154127	f
2	72	54	-1	0	1736738159474	f
2	72	61	-1	0	1736738161414	f
2	72	3	-1	0	1736738172491	f
2	72	66	-1	0	1736738175772	f
2	72	70	-1	0	1736738179572	f
2	72	29	0	0	1736738188802	f
2	72	51	-1	0	1736738192078	f
2	72	31	-1	0	1736738196568	f
2	72	50	-1	0	1736738200567	f
2	72	62	0	0	1736738206317	f
2	72	43	-1	0	1736738208979	f
2	72	12	-1	0	1736738214523	f
2	72	47	0	0	1736738224309	f
2	72	23	-1	0	1736738227777	f
2	72	28	-1	0	1736738234281	f
2	72	37	-1	0	1736738239489	f
2	72	17	-1	0	1736738242422	f
2	72	32	-1	0	1736738250658	f
2	72	44	-1	0	1736738252991	f
2	72	22	-1	0	1736738260853	f
2	72	35	-1	0	1736738273362	f
2	72	59	-1	0	1736738282779	f
2	72	20	-1	0	1736738284741	f
2	72	53	-1	0	1736738293675	f
2	72	48	-1	0	1736738301415	f
2	72	65	-1	0	1736738329923	f
2	77	17	-1	0	1736744315277	f
2	77	72	-1	0	1736744340569	f
5	0	7	0	0	1736791367939	f
5	5	9	0	0	1736794883233	f
5	5	11	0	0	1736794886167	f
5	10	1	0	0	1736797104164	f
5	10	16	0	0	1736797126414	f
5	10	15	-1	0	1736797172074	f
5	10	19	-1	0	1736797179073	f
5	10	2	0	0	1736797185600	f
5	11	13	-1	0	1736797829001	f
5	11	20	-1	0	1736797851410	f
5	11	4	-1	0	1736797867036	f
5	11	10	1	0	1736797878147	f
5	11	17	-1	0	1736797896464	f
5	11	12	-1	0	1736797900727	f
5	14	8	-1	0	1736803887981	f
5	17	4	-1	0	1736809832972	f
5	8	21	-1	0	1736809834956	f
5	8	22	-1	0	1736809839089	f
2	78	68	0	0	1736827254403	f
2	78	31	-1	0	1736827261276	f
2	78	69	1	0	1736827741625	f
2	78	32	-1	0	1736827747930	f
2	78	20	-1	0	1736827762376	f
2	81	35	-1	0	1736879927097	f
2	81	31	-1	0	1736879944361	f
2	81	53	-1	0	1736879961422	f
2	82	32	-1	0	1736880686388	f
2	82	50	-1	0	1736881000294	f
2	82	24	1	0	1736881021121	f
2	86	71	0	0	1736882422191	f
2	87	53	-1	0	1736916389064	f
2	87	33	0	0	1736916421684	f
2	87	52	1	0	1736917234959	f
2	87	51	0	0	1736917253991	f
2	88	38	-1	0	1736994739536	f
2	89	36	0	0	1737002065790	f
2	90	13	-1	0	1737041238541	f
2	90	64	-1	0	1737041252337	f
2	90	35	-1	0	1737041256107	f
2	90	17	-1	0	1737041267616	f
2	90	12	-1	0	1737041650801	f
2	91	1	1	0	1737265238157	f
2	91	2	-1	0	1737265244076	f
2	92	50	1	0	1737467647379	f
9	5	0	-1	0	1739378652091	f
12	0	14	-1	0	1739382213069	f
12	4	4	-1	0	1739454485169	f
12	5	0	-1	0	1739454500394	f
12	3	19	-1	0	1739454982281	f
12	20	28	1	0	1739455523004	f
12	20	23	-1	0	1739455531052	f
12	10	8	1	0	1739455828884	f
9	11	33	-1	0	1739456366347	f
12	20	11	-1	0	1739456435124	f
12	12	27	1	0	1739456437133	f
12	4	11	0	0	1739456437804	f
12	4	30	0	0	1739456444595	f
12	22	30	0	0	1739456473555	f
12	22	6	-1	0	1739456501526	f
12	32	6	-1	0	1739456504788	f
9	8	35	1	0	1739456798413	f
12	33	19	-1	0	1739457844485	f
12	34	33	-1	0	1739458524120	f
12	21	1	-1	0	1739459449824	f
12	21	2	-1	0	1739459454984	f
12	21	3	-1	0	1739459457624	f
12	5	1	-1	0	1739459469881	f
12	21	7	-1	0	1739459471347	f
12	21	33	0	0	1739459493929	f
12	13	8	1	0	1739459795516	f
12	38	34	-1	0	1739459799717	f
12	22	1	-1	0	1739459812637	f
12	36	8	1	0	1739459821253	f
12	41	34	-1	0	1739459827595	f
12	10	34	-1	0	1739459829851	f
12	40	32	1	0	1739459833051	f
12	7	3	-1	0	1739459847815	f
12	40	13	-1	0	1739459850728	f
12	38	28	1	0	1739459865774	f
12	16	19	-1	0	1739459876404	f
12	40	29	-1	0	1739459876632	f
12	7	7	-1	0	1739459884569	f
12	44	1	-1	0	1739460077353	f
12	44	30	0	0	1739460099546	f
12	44	31	-1	0	1739460122768	f
12	45	7	-1	0	1739460817172	f
12	45	29	0	0	1739460854224	f
12	50	27	-1	0	1739461921889	f
9	14	37	-1	0	1739462751613	f
12	54	29	-1	0	1739463025667	f
12	54	30	1	0	1739463034984	f
12	18	35	-1	0	1739463771284	f
12	58	25	-1	0	1739464197005	f
12	40	22	-1	0	1739464223306	f
12	29	34	1	0	1739464224050	f
12	15	31	-1	0	1739464238117	f
12	56	25	-1	0	1739464239747	f
12	35	7	1	0	1739464260544	f
12	29	25	-1	0	1739464264549	f
12	21	37	0	0	1739464867467	f
12	17	21	-1	0	1739464871290	f
12	34	21	0	0	1739464872103	f
12	29	37	-1	0	1739465188860	f
12	22	22	0	0	1739465194632	f
12	53	34	0	0	1739465211588	f
12	53	21	0	0	1739465215714	f
12	53	36	-1	0	1739465220081	f
12	29	21	-1	0	1739465232760	f
12	15	18	-1	0	1739465945548	f
12	41	37	-1	0	1739465946812	f
12	15	21	-1	0	1739465951967	f
12	22	35	-1	0	1739465954699	f
12	41	21	1	0	1739465957258	f
12	59	6	-1	0	1739465962089	f
12	59	3	-1	0	1739465964792	f
12	59	38	-1	0	1739465966741	f
12	49	37	-1	0	1739465967082	f
12	22	37	-1	0	1739465972076	f
12	59	26	-1	0	1739465972275	f
12	59	2	-1	0	1739465974211	f
12	59	13	-1	0	1739466022671	f
12	35	39	1	0	1739466332679	f
12	17	40	-1	0	1739468134551	f
12	60	19	-1	0	1739595989018	f
12	60	46	-1	0	1739596748921	f
2	72	27	1	0	1736738311443	f
2	72	56	-1	0	1736738678722	f
2	72	25	-1	0	1736738681756	f
2	72	63	-1	0	1736738689261	f
2	72	42	-1	0	1736738694331	f
2	72	49	0	0	1736738703951	f
2	72	9	-1	0	1736738706699	f
2	72	46	-1	0	1736738710826	f
2	72	36	0	0	1736738715767	f
2	72	41	0	0	1736738723967	f
2	72	57	0	0	1736738728239	f
2	72	24	0	0	1736738738305	f
2	61	72	1	0	1736738751054	f
2	61	71	-1	0	1736738757202	f
2	74	68	-1	0	1736738953592	f
2	74	72	-1	0	1736738962633	f
2	74	2	-1	0	1736738966226	f
2	74	30	1	0	1736738976247	f
2	74	21	-1	0	1736738981446	f
2	74	64	-1	0	1736738988746	f
2	74	1	1	0	1736738990946	f
2	74	48	0	0	1736738997566	f
2	74	6	1	0	1736739000306	f
2	74	63	-1	0	1736739004206	f
2	74	56	-1	0	1736739021986	f
2	74	71	1	0	1736739035652	f
2	74	36	-1	0	1736739040568	f
2	75	1	-1	0	1736740161609	f
2	75	2	0	0	1736740166911	f
2	75	6	1	0	1736740191399	f
2	75	30	0	0	1736740199526	f
2	75	72	0	0	1736740210094	f
2	75	35	-1	0	1736740215899	f
2	75	31	-1	0	1736740220531	f
2	75	65	-1	0	1736740228399	f
2	75	17	-1	0	1736740231549	f
2	75	25	-1	0	1736740234133	f
2	75	66	-1	0	1736740238784	f
2	75	47	-1	0	1736740247760	f
2	75	9	-1	0	1736740251205	f
2	75	71	0	0	1736740267207	f
2	75	68	-1	0	1736740272844	f
2	75	38	0	0	1736740279977	f
2	75	64	0	0	1736740283813	f
2	75	58	-1	0	1736740289976	f
2	75	33	-1	0	1736740295508	f
2	75	67	0	0	1736740302063	f
2	75	13	-1	0	1736740304664	f
2	75	3	-1	0	1736740308396	f
2	75	69	-1	0	1736740312083	f
2	75	15	-1	0	1736740318134	f
2	75	70	-1	0	1736740321834	f
2	75	63	-1	0	1736740326519	f
2	75	34	-1	0	1736740332078	f
2	75	59	-1	0	1736740352572	f
2	75	12	0	0	1736740356456	f
2	75	22	1	0	1736740368422	f
2	75	28	-1	0	1736740375392	f
2	75	37	-1	0	1736740386329	f
2	75	53	-1	0	1736740390047	f
2	75	23	0	0	1736740395478	f
2	75	54	-1	0	1736740400830	f
2	75	29	-1	0	1736740407586	f
2	75	48	-1	0	1736740415018	f
2	75	46	-1	0	1736740419749	f
2	77	25	-1	0	1736744320344	f
2	77	68	-1	0	1736744325420	f
5	0	8	0	0	1736791498691	f
5	5	12	0	0	1736794906761	f
5	11	18	-1	0	1736797833114	f
5	11	19	-1	0	1736797847041	f
5	11	14	-1	0	1736797859337	f
5	11	15	-1	0	1736797875975	f
5	11	7	-1	0	1736797881628	f
5	11	9	1	0	1736797902387	f
5	11	2	1	0	1736797905966	f
5	14	3	1	0	1736803929285	f
5	14	10	1	0	1736803939617	f
5	14	12	1	0	1736803948268	f
5	18	10	1	0	1736814068084	f
2	78	43	-1	0	1736827256734	f
2	78	12	-1	0	1736827263524	f
2	78	38	1	0	1736827272022	f
2	78	24	0	0	1736827924996	f
2	81	69	-1	0	1736879932364	f
2	81	40	0	0	1736879941077	f
2	81	33	-1	0	1736879949247	f
2	81	12	-1	0	1736879958198	f
2	82	53	-1	0	1736880705140	f
2	82	60	-1	0	1736880739009	f
2	82	63	-1	0	1736880743631	f
2	84	17	-1	0	1736880754689	f
2	82	49	0	0	1736881031402	f
2	86	53	-1	0	1736882424951	f
2	87	44	-1	0	1736916391392	f
2	87	10	-1	0	1736916406026	f
2	87	13	-1	0	1736917241390	f
2	87	12	0	0	1736917272928	f
2	88	12	-1	0	1736994743929	f
2	89	71	0	0	1737002076928	f
2	89	6	0	0	1737002079542	f
2	90	37	-1	0	1737041264741	f
2	90	69	-1	0	1737041272436	f
2	90	38	-1	0	1737041278224	f
2	90	39	1	0	1737041663568	f
2	90	23	0	0	1737041682232	f
2	91	70	-1	0	1737265252354	f
2	91	68	-1	0	1737265282891	f
2	91	65	-1	0	1737265292194	f
2	91	61	0	0	1737265304663	f
2	91	63	-1	0	1737265322262	f
2	91	35	-1	0	1737265335327	f
2	91	56	0	0	1737265351328	f
2	92	62	-1	0	1737467674727	f
9	4	0	-1	0	1739378716668	f
12	0	15	-1	0	1739382223352	f
12	5	17	-1	0	1739454492656	f
12	5	4	-1	0	1739454496859	f
12	10	19	-1	0	1739454990485	f
12	28	17	-1	0	1739455006171	f
12	28	4	-1	0	1739455018446	f
12	28	23	-1	0	1739455035130	f
12	20	19	-1	0	1739455540024	f
12	20	29	-1	0	1739455835475	f
12	20	8	1	0	1739455842145	f
12	11	11	-1	0	1739456382376	f
12	12	23	-1	0	1739456415238	f
9	8	32	1	0	1739456812426	f
12	33	0	0	0	1739457886608	f
12	33	8	1	0	1739457894628	f
12	34	8	-1	0	1739458565292	f
12	34	2	-1	0	1739458574845	f
12	18	1	-1	0	1739459450023	f
12	22	13	-1	0	1739459796662	f
12	20	32	1	0	1739459797107	f
12	38	3	-1	0	1739459802568	f
\.


--
-- Data for Name: votes_latest_unique; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.votes_latest_unique (zid, pid, tid, vote, weight_x_32767, modified) FROM stdin;
2	0	0	0	0	1736645112983
2	0	1	0	0	1736645326845
2	0	2	0	0	1736645419842
2	0	3	0	0	1736645624667
2	0	4	0	0	1736645674996
2	0	5	0	0	1736645696284
2	0	6	0	0	1736645712966
2	0	7	0	0	1736645758830
2	0	8	0	0	1736645773730
2	0	9	0	0	1736645801425
2	0	10	0	0	1736645807880
2	0	11	0	0	1736645836839
2	0	12	0	0	1736645874952
2	0	13	0	0	1736646064899
2	0	14	0	0	1736646075850
2	0	15	0	0	1736646664423
2	0	16	0	0	1736647227843
2	0	17	0	0	1736647444998
2	0	18	0	0	1736647466580
2	0	19	0	0	1736647514719
2	0	20	0	0	1736647550812
3	0	0	0	0	1736648104619
3	1	1	-1	0	1736648226888
3	1	0	-1	0	1736648231383
2	0	21	0	0	1736649660268
2	0	22	0	0	1736649697444
2	0	23	0	0	1736649752807
2	0	24	0	0	1736649905765
2	0	25	0	0	1736649945157
3	2	2	-1	0	1736651755530
3	2	1	-1	0	1736651756417
3	2	0	1	0	1736651757184
3	3	3	-1	0	1736651769936
3	3	2	-1	0	1736651772070
3	3	0	1	0	1736651773027
3	3	1	-1	0	1736651774016
3	4	4	-1	0	1736651789172
3	4	2	-1	0	1736651790756
3	4	1	1	0	1736651791283
3	4	0	-1	0	1736651792289
3	4	3	1	0	1736651792955
3	5	5	-1	0	1736651807350
3	5	4	-1	0	1736651808171
3	5	1	1	0	1736651808816
3	5	3	-1	0	1736651809966
3	5	0	-1	0	1736651810459
3	5	2	-1	0	1736651811204
3	6	6	-1	0	1736651834055
3	6	3	1	0	1736651835848
3	6	2	-1	0	1736651836945
3	6	4	1	0	1736651837489
3	6	1	-1	0	1736651838311
3	6	0	-1	0	1736651839495
3	6	5	1	0	1736651840729
2	1	26	-1	0	1736652321611
2	2	2	0	0	1736652375081
2	2	6	0	0	1736652376035
2	2	1	0	0	1736652377019
2	2	23	0	0	1736652378011
2	3	23	-1	0	1736653653755
2	4	2	0	0	1736653654778
2	4	6	1	0	1736653658066
2	3	2	-1	0	1736653660734
2	4	1	1	0	1736653660985
2	3	1	1	0	1736653662981
2	4	23	-1	0	1736653666308
2	3	25	0	0	1736653668593
2	4	5	-1	0	1736653668848
2	3	6	1	0	1736653670654
2	4	13	-1	0	1736653671431
2	3	21	-1	0	1736653674691
2	4	11	-1	0	1736653675797
2	3	13	-1	0	1736653677713
2	4	3	-1	0	1736653678665
2	3	5	-1	0	1736653680029
2	4	15	-1	0	1736653681394
2	4	21	-1	0	1736653684360
2	3	24	1	0	1736653684643
2	4	20	-1	0	1736653687421
2	4	17	-1	0	1736653691263
2	3	11	0	0	1736653694323
2	3	3	-1	0	1736653696932
2	4	24	0	0	1736653698076
2	4	9	-1	0	1736653701567
2	3	20	-1	0	1736653703041
2	4	10	-1	0	1736653703597
2	3	15	-1	0	1736653706032
2	3	17	-1	0	1736653708640
2	3	9	-1	0	1736653711573
2	4	22	0	0	1736653712439
2	3	10	-1	0	1736653713721
2	4	12	-1	0	1736653715561
2	4	25	-1	0	1736653718721
2	3	22	0	0	1736653722082
2	3	12	-1	0	1736653726343
2	5	24	1	0	1736654381078
2	5	20	-1	0	1736654390375
2	5	10	-1	0	1736654393159
2	5	1	1	0	1736654395817
2	5	2	1	0	1736654402406
2	5	25	-1	0	1736654408329
2	5	17	-1	0	1736654410766
2	5	15	-1	0	1736654413648
2	5	21	-1	0	1736654417670
2	5	11	-1	0	1736654422591
2	5	5	-1	0	1736654425958
2	5	6	-1	0	1736654428467
2	5	12	-1	0	1736654433551
2	5	9	-1	0	1736654435036
2	5	13	-1	0	1736654448845
2	5	3	-1	0	1736654452251
2	5	23	0	0	1736654469613
2	5	22	-1	0	1736654473757
2	6	17	-1	0	1736654517943
2	6	2	0	0	1736654523240
2	6	11	0	0	1736654535314
2	6	5	-1	0	1736654537136
2	6	12	-1	0	1736654539724
2	6	3	-1	0	1736654541942
2	6	1	1	0	1736654544375
2	6	10	-1	0	1736654553893
2	6	21	-1	0	1736654557532
2	6	20	-1	0	1736654559611
2	6	6	1	0	1736654561269
2	7	17	-1	0	1736654561446
2	6	15	-1	0	1736654563427
2	7	6	1	0	1736654564424
2	6	9	-1	0	1736654565069
2	6	24	-1	0	1736654568421
2	7	2	0	0	1736654570771
2	6	22	1	0	1736654571465
2	7	20	-1	0	1736654572802
2	6	13	-1	0	1736654573404
2	6	25	-1	0	1736654574875
2	7	1	1	0	1736654574949
2	7	13	-1	0	1736654581232
2	7	5	-1	0	1736654583111
2	7	23	-1	0	1736654586999
2	7	11	0	0	1736654591384
2	7	12	-1	0	1736654594003
2	7	3	-1	0	1736654597138
2	7	22	0	0	1736654601614
2	7	25	-1	0	1736654604021
2	7	9	-1	0	1736654605685
2	7	10	-1	0	1736654607051
2	7	21	-1	0	1736654611798
2	7	24	0	0	1736654618416
2	7	15	-1	0	1736654621570
2	8	15	-1	0	1736654654183
2	8	9	-1	0	1736654656664
2	8	1	1	0	1736654658624
2	8	3	-1	0	1736654660736
2	8	6	1	0	1736654662688
2	8	20	-1	0	1736654664159
2	8	5	-1	0	1736654665354
2	8	21	-1	0	1736654667561
2	8	2	-1	0	1736654670624
2	8	10	-1	0	1736654672389
2	8	17	-1	0	1736654678308
2	75	40	-1	0	1736740427591
2	75	60	1	0	1736740467335
2	75	55	-1	0	1736740475149
2	75	45	-1	0	1736740503655
2	75	44	-1	0	1736740510675
2	75	20	-1	0	1736740515843
2	75	41	-1	0	1736740526376
2	75	36	-1	0	1736740530594
2	75	49	0	0	1736740535779
2	59	73	-1	0	1736744822886
5	1	11	1	0	1736794360564
5	1	4	-1	0	1736794406921
5	1	7	-1	0	1736794414061
5	7	13	-1	0	1736795556253
5	7	5	-1	0	1736795562525
5	8	13	-1	0	1736795571282
5	13	4	-1	0	1736802801726
5	13	18	-1	0	1736802824139
5	13	20	-1	0	1736802845797
5	16	4	-1	0	1736807147744
5	20	16	-1	0	1736814464040
5	20	1	0	0	1736814466260
5	20	11	0	0	1736814475084
5	20	21	-1	0	1736814490631
5	20	12	0	0	1736814527889
5	20	17	-1	0	1736814548204
5	20	18	0	0	1736814557075
5	20	7	-1	0	1736814591545
5	20	19	-1	0	1736814630916
2	78	21	-1	0	1736827638714
2	79	3	-1	0	1736879846019
2	79	20	-1	0	1736879858719
2	81	1	1	0	1736879876179
2	79	59	-1	0	1736879883898
2	81	6	1	0	1736879891278
2	82	10	-1	0	1736880661640
2	82	50	-1	0	1736881000294
2	82	24	1	0	1736881021121
2	86	28	-1	0	1736882473263
2	86	15	-1	0	1736882497487
2	86	69	-1	0	1736882505754
2	86	50	-1	0	1736882515620
2	86	12	-1	0	1736882534081
2	86	23	1	0	1736882548800
2	87	40	-1	0	1736916466680
2	87	72	-1	0	1736916481774
2	87	59	0	0	1736916496530
2	87	57	0	0	1736917361935
2	88	46	-1	0	1736994937617
2	88	21	-1	0	1736994954722
2	89	28	-1	0	1737002425585
2	89	54	-1	0	1737002431557
2	89	5	1	0	1737002455314
2	90	57	0	0	1737041484842
2	90	58	1	0	1737041896412
2	90	62	-1	0	1737041928640
2	90	24	1	0	1737041950000
2	92	67	-1	0	1737467406013
2	92	10	-1	0	1737467420103
2	92	38	-1	0	1737467424375
2	92	3	-1	0	1737467435775
2	92	73	-1	0	1737467441002
2	92	15	-1	0	1737467444685
2	92	51	-1	0	1737467466957
2	92	17	-1	0	1737467472633
2	92	56	1	0	1737467498072
2	92	23	-1	0	1737467510742
2	92	41	-1	0	1737467520798
2	92	21	-1	0	1737467525631
2	92	11	-1	0	1737467534191
2	92	59	1	0	1737467551415
2	92	54	-1	0	1737467568441
2	92	25	-1	0	1737467585144
2	92	36	-1	0	1737467603893
9	6	5	-1	0	1739381383056
12	0	26	-1	0	1739382325355
12	9	17	-1	0	1739454641030
12	3	17	-1	0	1739454651907
12	4	28	-1	0	1739455293848
12	3	27	-1	0	1739455304555
12	3	28	0	0	1739455310256
12	30	23	-1	0	1739455691660
12	31	8	1	0	1739456187429
12	31	4	-1	0	1739456193829
12	30	6	-1	0	1739456720790
12	30	11	-1	0	1739456736753
12	3	29	0	0	1739456739097
12	33	28	1	0	1739457802969
9	12	37	-1	0	1739457820994
9	12	31	0	0	1739457836653
12	34	7	0	0	1739458497159
12	34	19	-1	0	1739458514954
12	18	1	-1	0	1739459450023
12	10	2	-1	0	1739459824214
12	36	11	1	0	1739459827021
12	20	7	-1	0	1739459829456
12	36	27	-1	0	1739459830407
12	38	29	-1	0	1739459830714
12	7	11	1	0	1739459844644
12	17	29	1	0	1739459846900
12	10	32	-1	0	1739459847177
12	17	1	1	0	1739459852368
12	17	33	0	0	1739459876012
12	44	28	1	0	1739460200590
12	46	31	-1	0	1739461177554
12	51	6	-1	0	1739462326301
12	41	25	-1	0	1739462912230
12	53	13	-1	0	1739462916524
12	11	13	-1	0	1739463538051
12	35	25	-1	0	1739464230075
12	56	7	0	0	1739464235636
12	29	2	-1	0	1739464235684
12	29	29	1	0	1739464244636
12	31	22	1	0	1739464273165
12	15	29	1	0	1739464297235
12	56	28	0	0	1739464297404
12	2	36	-1	0	1739464297566
12	5	21	-1	0	1739464912417
12	11	37	-1	0	1739464912682
12	16	21	-1	0	1739464924603
12	2	37	0	0	1739464942283
12	8	37	1	0	1739465419919
12	8	14	-1	0	1739465443577
12	34	39	0	0	1739466239289
12	34	38	0	0	1739466254335
12	17	40	-1	0	1739468134551
12	60	23	-1	0	1739596125340
2	8	25	-1	0	1736654673708
2	8	12	-1	0	1736654675141
2	75	42	-1	0	1736740430807
2	75	39	0	0	1736740442110
2	75	56	-1	0	1736740449545
2	59	69	1	0	1736744827997
5	1	1	-1	0	1736794365780
5	8	7	-1	0	1736795556753
5	7	16	0	0	1736795565854
5	7	4	-1	0	1736795572050
5	13	12	0	0	1736802835124
5	13	19	-1	0	1736802882816
5	16	22	-1	0	1736807278401
5	20	3	0	0	1736814468870
5	20	9	0	0	1736814473903
5	20	8	-1	0	1736814482011
5	20	5	-1	0	1736814533573
5	20	20	-1	0	1736814537315
2	78	10	-1	0	1736827643874
2	79	34	-1	0	1736879865243
2	82	34	0	0	1736880679615
2	82	43	1	0	1736880696490
2	82	66	0	0	1736880718752
2	82	49	0	0	1736881031402
2	86	3	-1	0	1736882477130
2	86	54	-1	0	1736882483861
2	86	58	-1	0	1736882509708
2	86	24	0	0	1736882522169
2	86	35	-1	0	1736882531880
2	86	36	-1	0	1736882539343
2	86	11	-1	0	1736882543022
2	86	20	1	0	1736882551916
2	86	29	1	0	1736882580338
2	86	56	0	0	1736882592305
2	86	31	-1	0	1736882596466
2	86	66	-1	0	1736882602994
2	86	44	-1	0	1736882607162
2	86	61	-1	0	1736882612593
2	86	62	-1	0	1736882626172
2	86	57	1	0	1736882632942
2	87	69	1	0	1736916474144
2	87	38	0	0	1736916488804
5	12	4	0	0	1736985941962
5	12	11	1	0	1736985953632
5	12	9	-1	0	1736985957448
5	12	8	1	0	1736985960490
5	12	6	-1	0	1736985969041
5	12	5	-1	0	1736985975682
2	88	64	1	0	1736994974252
2	89	69	-1	0	1737002435771
2	89	11	-1	0	1737002439507
2	89	66	-1	0	1737002466244
2	90	31	-1	0	1737041490353
2	90	61	1	0	1737041903768
2	90	49	1	0	1737041915214
2	92	65	-1	0	1737467506843
2	92	69	-1	0	1737467523286
9	6	6	-1	0	1739381406840
12	0	27	-1	0	1739382331427
12	10	17	-1	0	1739454654385
12	11	4	-1	0	1739454656462
12	11	17	-1	0	1739454662186
12	12	4	-1	0	1739454668413
12	17	17	-1	0	1739454705313
12	4	27	-1	0	1739455315167
12	4	8	1	0	1739455719577
12	31	19	0	0	1739456201740
12	3	30	0	0	1739456747947
12	3	6	-1	0	1739456758975
9	12	36	-1	0	1739457808999
9	12	32	-1	0	1739457814545
12	33	27	1	0	1739457827501
9	12	38	-1	0	1739457829563
9	12	30	0	0	1739457832307
12	34	33	-1	0	1739458524120
12	21	34	-1	0	1739459489009
12	2	1	-1	0	1739459573849
12	17	2	-1	0	1739459835188
12	10	13	-1	0	1739459839774
12	38	4	0	0	1739459848004
12	38	7	1	0	1739459855816
12	27	29	1	0	1739459883722
12	17	13	-1	0	1739459887186
12	16	27	-1	0	1739459887453
12	42	33	0	0	1739459887811
12	41	27	-1	0	1739459888769
12	27	6	-1	0	1739459890364
12	27	34	1	0	1739459895964
12	40	33	1	0	1739459898351
12	35	28	0	0	1739459900007
12	44	27	-1	0	1739460204144
12	45	28	-1	0	1739461211168
12	51	7	-1	0	1739462339588
12	51	4	-1	0	1739462348812
12	51	30	-1	0	1739462367081
12	31	25	-1	0	1739462932029
12	14	14	0	0	1739462934749
12	53	23	-1	0	1739462939358
12	54	13	-1	0	1739462946044
12	54	1	-1	0	1739462949587
12	31	31	-1	0	1739462951274
12	31	33	-1	0	1739462959850
12	31	7	1	0	1739462975701
12	2	22	-1	0	1739463598363
12	20	20	-1	0	1739464234763
12	20	36	-1	0	1739464242051
12	35	14	-1	0	1739464249554
12	29	3	-1	0	1739464258836
12	15	32	-1	0	1739464270028
12	15	1	0	0	1739464286059
12	56	32	0	0	1739464291032
12	2	18	-1	0	1739464912637
12	8	20	-1	0	1739465452037
12	31	18	-1	0	1739466276547
12	31	37	0	0	1739466280360
12	31	39	-1	0	1739466285493
12	3	23	-1	0	1739467872392
12	3	40	-1	0	1739468189521
12	60	43	-1	0	1739596156241
2	8	23	-1	0	1736654676936
2	8	24	-1	0	1736654681613
2	8	13	-1	0	1736654683212
2	8	11	-1	0	1736654684965
2	8	22	-1	0	1736654686206
2	9	2	0	0	1736654747999
2	9	23	-1	0	1736654758677
2	9	5	-1	0	1736654763396
2	9	22	1	0	1736654786325
2	9	17	0	0	1736654799152
2	9	10	-1	0	1736654807086
2	9	15	-1	0	1736654812305
2	9	21	-1	0	1736654818076
2	9	24	-1	0	1736654829037
2	9	1	0	0	1736654833505
2	9	12	1	0	1736654852894
2	10	9	-1	0	1736654854131
2	9	6	0	0	1736654856254
2	10	15	-1	0	1736654858297
2	10	6	0	0	1736654861114
2	9	25	-1	0	1736654863730
2	10	11	-1	0	1736654864298
2	10	1	0	0	1736654868556
2	10	2	0	0	1736654872487
2	9	9	0	0	1736654879187
2	10	20	0	0	1736654880325
2	9	3	1	0	1736654884218
2	10	24	0	0	1736654885445
2	9	13	1	0	1736654893501
2	10	12	-1	0	1736654896375
2	10	22	0	0	1736654899421
2	9	11	0	0	1736654901331
2	10	17	-1	0	1736654901886
2	10	3	-1	0	1736654909936
2	10	23	-1	0	1736654912188
2	10	10	-1	0	1736654915413
2	9	20	1	0	1736654917491
2	10	13	-1	0	1736654919374
2	10	25	-1	0	1736654922864
2	11	20	-1	0	1736654927215
2	11	1	1	0	1736654963533
2	11	2	1	0	1736654966364
2	11	5	-1	0	1736654968564
2	11	12	-1	0	1736654971155
2	11	6	1	0	1736654972952
2	11	21	-1	0	1736654976615
2	10	21	-1	0	1736654979519
2	10	5	-1	0	1736654982610
2	11	24	0	0	1736654991159
2	11	22	-1	0	1736655012782
2	11	15	-1	0	1736655015411
2	11	13	-1	0	1736655017621
2	11	10	-1	0	1736655022390
2	11	27	-1	0	1736655065841
2	11	23	-1	0	1736655079838
2	11	9	-1	0	1736655082239
2	11	25	-1	0	1736655084498
2	11	17	-1	0	1736655086360
2	11	3	-1	0	1736655089574
2	11	11	-1	0	1736655097908
2	10	28	-1	0	1736655163874
2	12	1	1	0	1736655204388
2	10	27	-1	0	1736655204712
2	12	6	1	0	1736655206583
2	12	2	-1	0	1736655218050
2	12	23	0	0	1736655227728
2	12	15	-1	0	1736655233091
2	12	22	-1	0	1736655242428
2	12	5	-1	0	1736655247304
2	12	27	-1	0	1736655256522
2	12	21	-1	0	1736655265814
2	12	13	-1	0	1736655270951
2	12	28	-1	0	1736655282742
2	13	24	0	0	1736655285270
2	13	6	-1	0	1736655287187
2	13	1	1	0	1736655290276
2	12	20	-1	0	1736655294865
2	13	2	0	0	1736655294943
2	12	9	-1	0	1736655296951
2	13	28	-1	0	1736655298265
2	13	13	-1	0	1736655300492
2	13	25	-1	0	1736655302429
2	13	12	-1	0	1736655304269
2	13	20	-1	0	1736655306685
2	13	5	-1	0	1736655308282
2	13	17	-1	0	1736655309871
2	13	27	-1	0	1736655312813
2	12	25	-1	0	1736655314430
2	13	9	-1	0	1736655316501
2	12	10	-1	0	1736655317448
2	13	10	-1	0	1736655318484
2	13	15	-1	0	1736655320398
2	12	17	-1	0	1736655321405
2	13	23	-1	0	1736655323697
2	13	21	-1	0	1736655325848
2	13	11	0	0	1736655329937
2	13	22	1	0	1736655334755
2	12	12	0	0	1736655335219
2	12	3	-1	0	1736655338421
2	13	3	-1	0	1736655339360
2	12	24	1	0	1736655350318
2	12	11	-1	0	1736655354213
2	14	28	-1	0	1736655354541
2	14	13	-1	0	1736655358501
2	14	1	1	0	1736655363787
2	14	6	1	0	1736655365778
2	14	2	1	0	1736655373550
2	14	5	-1	0	1736655375803
2	14	21	-1	0	1736655381260
2	14	27	-1	0	1736655405705
2	14	20	-1	0	1736655410626
2	14	23	-1	0	1736655429376
2	14	12	-1	0	1736655431642
2	13	29	-1	0	1736655435925
2	12	30	-1	0	1736655444571
2	12	29	1	0	1736655464263
2	13	31	-1	0	1736655495157
2	13	30	1	0	1736655506625
2	14	24	1	0	1736655533911
2	14	9	-1	0	1736655536674
2	14	30	1	0	1736655540684
2	14	25	-1	0	1736655543945
2	13	32	-1	0	1736655563299
2	14	31	-1	0	1736655586240
2	14	29	-1	0	1736655592279
2	14	10	-1	0	1736655595042
2	14	15	-1	0	1736655639130
2	14	32	-1	0	1736655647372
2	14	22	1	0	1736655660068
2	14	17	-1	0	1736655662637
2	14	11	0	0	1736655668464
2	14	3	-1	0	1736655671945
2	0	29	-1	0	1736655942814
2	0	30	1	0	1736655946778
2	0	28	-1	0	1736655952130
2	0	27	-1	0	1736655960445
2	0	31	-1	0	1736655964126
2	0	32	-1	0	1736655968026
2	15	2	-1	0	1736656291010
2	15	32	0	0	1736656300973
2	15	20	0	0	1736656309560
2	15	1	0	0	1736656312479
2	15	29	-1	0	1736656317166
2	15	30	-1	0	1736656322108
2	15	6	1	0	1736656327295
2	15	11	-1	0	1736656330061
2	15	27	0	0	1736656337375
2	16	15	-1	0	1736656341096
2	16	1	0	0	1736656348157
2	16	6	1	0	1736656355644
2	15	31	-1	0	1736656361596
2	16	31	-1	0	1736656363282
2	16	21	-1	0	1736656366380
2	16	2	1	0	1736656368896
2	15	28	0	0	1736656370543
2	15	13	-1	0	1736656373097
2	15	3	-1	0	1736656376163
2	15	12	-1	0	1736656378510
2	15	5	0	0	1736656384097
2	75	52	-1	0	1736740433871
2	75	27	-1	0	1736740445425
2	75	5	-1	0	1736740452180
4	0	0	0	0	1736754941030
5	1	10	1	0	1736794388778
5	1	8	-1	0	1736794398840
5	8	1	0	0	1736795561257
5	7	1	0	0	1736795564893
5	8	12	0	0	1736795582689
5	9	13	-1	0	1736795587135
5	8	10	0	0	1736795592702
5	7	8	-1	0	1736795594442
5	8	14	-1	0	1736795605412
5	7	7	-1	0	1736795608794
5	8	16	0	0	1736795611542
5	7	6	0	0	1736795619869
5	7	12	0	0	1736795621508
5	7	10	-1	0	1736795629469
5	9	7	-1	0	1736795725789
5	9	5	0	0	1736795740493
5	9	15	-1	0	1736795745529
5	8	17	-1	0	1736795787959
5	13	8	-1	0	1736802841816
5	13	3	0	0	1736802848359
5	9	21	-1	0	1736808499265
2	78	1	1	0	1736826738934
2	78	2	0	0	1736826750791
2	78	56	-1	0	1736827671097
2	78	29	0	0	1736827693656
2	78	39	0	0	1736827715800
2	78	22	0	0	1736827733101
2	78	47	-1	0	1736827755812
2	79	68	-1	0	1736879871226
2	81	2	0	0	1736879881456
2	79	21	-1	0	1736879889879
2	82	32	-1	0	1736880686388
2	82	57	0	0	1736881039895
2	86	21	-1	0	1736882486907
2	86	72	-1	0	1736882495016
2	86	51	1	0	1736882500514
2	86	64	1	0	1736882526497
2	86	22	-1	0	1736882541115
2	86	60	-1	0	1736882567609
2	86	63	-1	0	1736882583717
2	86	17	-1	0	1736882605116
2	86	52	-1	0	1736882609494
2	86	46	-1	0	1736882617797
2	87	45	0	0	1736916519361
5	12	3	-1	0	1736985944482
5	12	22	-1	0	1736985950470
5	12	21	-1	0	1736985951666
5	12	14	-1	0	1736985964966
5	12	18	-1	0	1736985970903
5	12	15	1	0	1736985974174
5	12	19	1	0	1736985976898
5	12	20	-1	0	1736985982519
2	88	40	-1	0	1736994979687
2	89	50	-1	0	1737002472212
2	90	63	-1	0	1737041493076
2	90	60	-1	0	1737041507293
2	90	52	-1	0	1737041918967
2	90	32	-1	0	1737041933064
2	92	64	-1	0	1737467553842
2	92	68	-1	0	1737467577405
2	92	31	-1	0	1737467588479
9	6	7	-1	0	1739381424062
9	5	13	-1	0	1739383570925
9	5	15	-1	0	1739383607556
9	5	18	-1	0	1739383629466
9	5	19	-1	0	1739383635993
9	5	20	-1	0	1739383641350
9	5	22	-1	0	1739383655592
9	5	23	-1	0	1739383662554
9	5	26	-1	0	1739383680900
9	5	27	-1	0	1739383688690
12	13	4	0	0	1739454671941
12	16	17	-1	0	1739454692408
12	13	0	1	0	1739454701523
12	18	0	-1	0	1739454704520
12	16	4	0	0	1739454709851
12	21	4	-1	0	1739454727361
12	20	4	-1	0	1739454730512
12	5	28	1	0	1739455304631
12	4	19	0	0	1739455308645
12	4	23	-1	0	1739455312505
12	4	29	0	0	1739455725166
12	31	0	-1	0	1739456206580
12	9	6	-1	0	1739456222703
12	9	30	1	0	1739456235556
12	9	11	1	0	1739456245559
12	3	11	0	0	1739456753522
12	33	4	1	0	1739457837212
9	12	29	0	0	1739457840146
12	34	8	-1	0	1739458565292
12	34	2	-1	0	1739458574845
12	2	13	-1	0	1739459578426
12	4	1	-1	0	1739459597931
12	7	23	-1	0	1739459856274
12	41	6	-1	0	1739459856905
12	17	31	-1	0	1739459857139
12	38	19	-1	0	1739459860967
12	17	30	0	0	1739459871768
12	38	30	-1	0	1739459875728
12	14	32	0	0	1739459880117
12	30	34	-1	0	1739459894071
12	35	30	1	0	1739459895166
12	43	2	-1	0	1739459904680
12	43	6	-1	0	1739459907655
12	8	31	-1	0	1739459911199
12	40	31	1	0	1739459912941
12	16	29	0	0	1739459913491
12	14	13	-1	0	1739459926655
12	17	11	0	0	1739459930574
12	43	31	-1	0	1739459930764
12	16	34	1	0	1739459934009
12	14	3	0	0	1739459937467
12	40	6	-1	0	1739459941461
12	43	30	1	0	1739459944181
12	43	29	-1	0	1739459946819
12	41	32	1	0	1739459946832
12	16	1	-1	0	1739459948926
12	40	4	-1	0	1739459949037
12	43	7	-1	0	1739459950644
12	43	28	1	0	1739459954103
12	16	13	-1	0	1739459966400
12	17	19	-1	0	1739459968274
12	14	7	0	0	1739459968393
12	40	34	1	0	1739459969200
12	14	2	-1	0	1739459972487
12	40	17	-1	0	1739459982184
12	16	7	0	0	1739459983751
12	14	6	0	0	1739459984470
12	14	19	0	0	1739459989797
12	40	0	1	0	1739459993092
12	14	8	0	0	1739459999520
12	8	6	1	0	1739460236459
12	8	32	1	0	1739460273114
12	44	8	0	0	1739460280482
12	45	8	1	0	1739461238939
12	51	29	-1	0	1739462358297
12	31	13	1	0	1739462936302
12	54	19	-1	0	1739462939491
12	2	20	-1	0	1739463643070
12	56	25	-1	0	1739464239747
12	35	7	1	0	1739464260544
12	29	25	-1	0	1739464264549
12	5	37	-1	0	1739464916497
12	16	18	-1	0	1739464917257
12	11	21	-1	0	1739464920835
12	21	18	-1	0	1739465535007
12	18	21	-1	0	1739465567014
12	30	26	0	0	1739466289928
12	0	41	0	0	1739468200953
12	60	36	1	0	1739596159095
12	60	22	0	0	1739596163801
2	15	9	-1	0	1736656380804
2	16	29	-1	0	1736656385903
2	15	23	0	0	1736656391154
2	16	24	1	0	1736656417118
2	15	25	-1	0	1736656429507
2	15	10	-1	0	1736656436023
2	15	24	0	0	1736656445837
2	16	23	-1	0	1736656450185
2	16	22	0	0	1736656461066
2	16	11	-1	0	1736656464115
2	75	10	-1	0	1736740470535
2	75	51	0	0	1736740480601
2	75	62	0	0	1736740507988
2	75	11	0	0	1736740520276
2	75	50	0	0	1736740538772
4	0	1	0	0	1736754959984
5	1	5	-1	0	1736794394966
5	8	15	-1	0	1736795577902
5	7	14	0	0	1736795584653
5	8	8	-1	0	1736795590490
5	8	11	0	0	1736795607731
5	8	9	0	0	1736795610501
5	8	4	-1	0	1736795620708
5	7	11	0	0	1736795622279
5	7	9	0	0	1736795627476
5	9	1	0	0	1736795730311
5	9	4	-1	0	1736795752172
5	9	18	-1	0	1736795934394
5	13	17	-1	0	1736802887269
5	13	14	-1	0	1736802903737
5	9	22	-1	0	1736808503732
2	78	23	-1	0	1736826743537
2	78	65	-1	0	1736827677162
2	78	46	-1	0	1736827708970
2	78	25	-1	0	1736827718733
2	78	51	0	0	1736827752047
2	78	3	-1	0	1736827760445
2	78	50	0	0	1736827775420
2	79	38	-1	0	1736879897543
2	79	9	1	0	1736879905014
2	82	53	-1	0	1736880705140
2	82	60	-1	0	1736880739009
2	82	63	-1	0	1736880743631
2	84	17	-1	0	1736880754689
2	85	1	1	0	1736881628303
2	85	39	-1	0	1736881653201
2	85	30	-1	0	1736881659352
2	85	21	-1	0	1736881669299
2	85	29	-1	0	1736881676917
2	85	9	-1	0	1736881712126
2	85	34	-1	0	1736881720934
2	86	55	-1	0	1736882570458
2	86	39	-1	0	1736882575784
2	86	43	-1	0	1736882581821
2	86	9	-1	0	1736882597891
2	86	5	1	0	1736882621847
2	86	49	-1	0	1736882629392
2	87	35	0	0	1736916534490
5	12	16	1	0	1736985946424
5	12	7	-1	0	1736985955665
5	12	2	-1	0	1736985958957
5	12	1	-1	0	1736985962003
5	12	17	1	0	1736985972598
5	12	13	-1	0	1736985980361
2	88	65	1	0	1736994997897
2	89	61	1	0	1737002530372
2	89	53	-1	0	1737002541788
2	90	40	-1	0	1737041497361
2	13	69	-1	0	1737062884671
2	13	71	0	0	1737062894410
2	92	60	-1	0	1737467560840
2	92	20	-1	0	1737467563093
2	92	22	-1	0	1737467574153
2	92	29	-1	0	1737467582328
9	6	8	-1	0	1739381437641
9	5	14	-1	0	1739383587033
9	5	16	-1	0	1739383617736
9	5	17	-1	0	1739383623337
9	5	24	-1	0	1739383668334
12	12	17	-1	0	1739454672253
12	20	17	-1	0	1739454718151
12	23	4	-1	0	1739454734196
12	24	17	-1	0	1739454734756
12	20	0	1	0	1739454738074
12	24	0	0	0	1739454762129
12	22	0	1	0	1739454787319
12	25	4	-1	0	1739454790955
12	14	4	-1	0	1739454827844
12	9	23	-1	0	1739455333718
12	2	29	1	0	1739455731886
12	30	4	-1	0	1739455731924
12	30	19	-1	0	1739455765025
12	11	8	-1	0	1739455765713
12	26	6	0	0	1739456234365
12	3	8	0	0	1739456765235
9	8	34	1	0	1739456787771
12	33	19	-1	0	1739457844485
12	34	3	1	0	1739458585976
12	2	2	-1	0	1739458592214
12	4	13	-1	0	1739459595407
12	40	19	1	0	1739459859046
12	41	19	-1	0	1739459863165
12	7	31	-1	0	1739459873932
12	39	33	-1	0	1739459875511
12	41	23	-1	0	1739459875690
12	27	33	-1	0	1739459877743
12	41	1	-1	0	1739459878473
12	42	3	-1	0	1739459894726
12	7	30	-1	0	1739459899793
12	16	33	-1	0	1739459904570
12	14	34	0	0	1739459905647
12	43	13	-1	0	1739459917920
12	14	29	0	0	1739459945673
12	17	27	0	0	1739459945803
12	17	6	-1	0	1739459950786
12	40	2	-1	0	1739459952139
12	14	27	-1	0	1739459952267
12	41	29	1	0	1739459959680
12	40	30	1	0	1739459962380
12	8	13	-1	0	1739460240074
12	8	34	-1	0	1739460245093
12	8	2	-1	0	1739460246808
12	8	33	-1	0	1739460260559
12	12	1	-1	0	1739461824474
12	51	32	0	0	1739462374839
12	51	34	1	0	1739462379752
12	51	8	0	0	1739462387574
12	54	7	0	0	1739462975962
12	18	22	-1	0	1739463776316
12	18	20	-1	0	1739463784179
12	5	36	-1	0	1739464254424
12	29	20	1	0	1739464274200
12	29	33	-1	0	1739464287884
12	29	30	1	0	1739464308616
12	2	21	-1	0	1739464928189
12	11	18	-1	0	1739464930971
12	31	26	-1	0	1739466290016
12	31	38	-1	0	1739466293585
12	5	41	-1	0	1739468239920
12	60	34	-1	0	1739596170467
12	18	26	-1	0	1739480968671
12	60	31	-1	0	1739596182854
12	60	13	0	0	1739596188324
12	60	27	1	0	1739596192743
12	60	20	0	0	1739596201594
2	16	30	1	0	1736656381491
2	16	20	-1	0	1736656394700
2	15	22	0	0	1736656395952
2	16	28	0	0	1736656404033
2	16	12	-1	0	1736656419656
2	15	17	-1	0	1736656424449
2	16	32	-1	0	1736656427453
2	16	9	-1	0	1736656429722
2	15	15	-1	0	1736656432524
2	16	27	-1	0	1736656432994
2	16	13	-1	0	1736656436395
2	15	21	-1	0	1736656439407
2	16	17	-1	0	1736656441296
2	16	25	-1	0	1736656455333
2	16	10	-1	0	1736656457108
2	16	5	0	0	1736656467519
2	16	3	-1	0	1736656471574
2	15	33	-1	0	1736656492202
2	13	33	0	0	1736656540549
2	15	34	-1	0	1736656618759
2	13	34	-1	0	1736656674264
2	15	35	-1	0	1736656776321
2	17	32	-1	0	1736656787614
2	17	1	0	0	1736656797031
2	17	31	-1	0	1736656802790
2	17	34	-1	0	1736656809735
2	17	27	-1	0	1736656827615
2	17	30	1	0	1736656833865
2	17	20	-1	0	1736656839875
2	17	28	-1	0	1736656846200
2	17	6	-1	0	1736656869129
2	17	2	-1	0	1736656873322
2	13	35	-1	0	1736656886636
2	17	33	-1	0	1736656888755
2	17	29	-1	0	1736656896382
2	17	13	-1	0	1736656908940
2	17	35	-1	0	1736656913668
2	17	17	-1	0	1736656926585
2	17	12	-1	0	1736656931131
2	17	9	-1	0	1736656934515
2	17	10	-1	0	1736656938415
2	17	11	-1	0	1736656943154
2	17	3	-1	0	1736656946671
2	17	21	-1	0	1736656950838
2	17	5	-1	0	1736656954045
2	17	22	-1	0	1736656959787
2	17	15	-1	0	1736656964188
2	17	23	-1	0	1736656966953
2	17	25	-1	0	1736656970097
2	17	24	0	0	1736656978206
2	18	29	-1	0	1736657017459
2	0	36	0	0	1736657019908
2	18	30	1	0	1736657023150
2	19	27	0	0	1736657028359
2	18	33	-1	0	1736657032336
2	19	1	1	0	1736657035693
2	19	12	-1	0	1736657041545
2	18	32	0	0	1736657051810
2	18	10	-1	0	1736657054845
2	18	6	1	0	1736657057554
2	18	1	1	0	1736657059531
2	18	12	-1	0	1736657062252
2	19	32	0	0	1736657062417
2	18	34	-1	0	1736657066998
2	19	30	-1	0	1736657070943
2	19	2	1	0	1736657081916
2	19	6	1	0	1736657084528
2	17	36	-1	0	1736657085511
2	19	28	-1	0	1736657093683
2	19	29	0	0	1736657104473
2	0	37	0	0	1736657112070
2	19	36	-1	0	1736657112790
2	19	34	-1	0	1736657121237
2	19	5	-1	0	1736657129636
2	17	37	-1	0	1736657129996
2	19	20	-1	0	1736657136437
2	19	22	-1	0	1736657152188
2	20	2	0	0	1736657159900
2	19	37	0	0	1736657168402
2	21	2	1	0	1736657170196
2	21	6	1	0	1736657172339
2	21	5	-1	0	1736657175195
2	21	30	1	0	1736657180554
2	20	15	-1	0	1736657183749
2	22	13	-1	0	1736657185873
2	21	13	0	0	1736657187188
2	21	1	1	0	1736657189022
2	22	30	-1	0	1736657193515
2	20	32	-1	0	1736657194251
2	21	22	1	0	1736657194284
2	21	31	1	0	1736657199231
2	22	2	-1	0	1736657200658
2	20	35	-1	0	1736657202091
2	21	34	-1	0	1736657205516
2	20	22	0	0	1736657206028
2	22	6	1	0	1736657207232
2	23	34	-1	0	1736657209910
2	21	29	0	0	1736657210200
2	24	30	1	0	1736657211869
2	22	35	-1	0	1736657212044
2	21	28	-1	0	1736657213666
2	22	1	0	0	1736657216599
2	21	33	-1	0	1736657217900
2	23	31	-1	0	1736657218231
2	20	6	1	0	1736657218450
2	24	6	1	0	1736657220088
2	23	1	1	0	1736657221948
2	20	28	-1	0	1736657222536
2	24	20	-1	0	1736657224566
2	22	34	-1	0	1736657224789
2	24	1	1	0	1736657227318
2	23	30	1	0	1736657228197
2	20	34	-1	0	1736657229439
2	20	5	-1	0	1736657232097
2	23	10	-1	0	1736657232247
2	23	21	-1	0	1736657236272
2	20	30	1	0	1736657237134
2	24	21	-1	0	1736657237480
2	23	9	-1	0	1736657238676
2	20	13	-1	0	1736657239871
2	22	27	1	0	1736657240279
2	23	6	1	0	1736657240383
2	20	1	1	0	1736657242260
2	13	36	-1	0	1736657242389
2	24	23	-1	0	1736657243438
2	22	25	-1	0	1736657244544
2	23	29	-1	0	1736657244838
2	13	37	-1	0	1736657246516
2	20	31	-1	0	1736657247578
2	22	12	-1	0	1736657247835
2	24	2	1	0	1736657247922
2	23	33	-1	0	1736657250443
2	20	20	-1	0	1736657251596
2	20	27	-1	0	1736657255562
2	22	29	-1	0	1736657256359
2	24	33	0	0	1736657258777
2	23	23	-1	0	1736657259073
2	22	36	-1	0	1736657262295
2	24	36	-1	0	1736657264844
2	22	33	-1	0	1736657269856
2	20	21	-1	0	1736657270103
2	23	35	-1	0	1736657271889
2	20	12	-1	0	1736657272787
2	22	20	1	0	1736657273285
2	24	28	0	0	1736657275337
2	23	2	1	0	1736657277207
2	20	36	-1	0	1736657278620
2	24	15	-1	0	1736657280310
2	23	20	-1	0	1736657282261
2	24	5	-1	0	1736657284094
2	20	37	-1	0	1736657284109
2	23	17	-1	0	1736657284399
2	20	10	-1	0	1736657287397
2	20	33	0	0	1736657296856
2	20	9	-1	0	1736657299202
2	23	37	-1	0	1736657301432
2	23	15	-1	0	1736657304760
2	20	3	-1	0	1736657305934
2	75	43	-1	0	1736740477651
2	75	32	0	0	1736740486788
4	0	2	0	0	1736755000132
5	1	3	-1	0	1736794401315
5	2	12	0	0	1736794422839
5	3	11	1	0	1736794459164
5	3	6	0	0	1736794492276
5	3	4	-1	0	1736794506867
5	3	1	1	0	1736794511569
5	3	2	1	0	1736794526338
5	8	3	0	0	1736795580541
5	7	2	0	0	1736795585785
5	7	15	0	0	1736795589435
5	9	11	0	0	1736795595556
5	8	2	0	0	1736795609135
5	7	3	0	0	1736795612694
5	8	5	-1	0	1736795624561
5	13	16	1	0	1736802889719
5	13	6	0	0	1736802930656
5	17	1	-1	0	1736809741560
2	78	6	-1	0	1736826753912
2	78	64	-1	0	1736827682045
2	81	35	-1	0	1736879927097
2	81	31	-1	0	1736879944361
2	81	53	-1	0	1736879961422
2	82	17	-1	0	1736880708411
2	85	2	-1	0	1736881632456
2	85	38	1	0	1736881641968
2	85	6	-1	0	1736881648255
2	85	70	-1	0	1736881657313
2	85	69	-1	0	1736881663197
2	85	67	-1	0	1736881681689
2	87	1	1	0	1736916054141
2	87	3	-1	0	1736916537644
2	88	1	1	0	1736993941583
2	88	52	-1	0	1736993956131
2	88	51	0	0	1736995006631
2	88	66	0	0	1736995021567
2	89	25	-1	0	1737002535436
2	89	59	-1	0	1737002551951
2	89	23	1	0	1737002568431
2	90	33	-1	0	1737041524641
2	90	11	0	0	1737041540237
2	13	73	-1	0	1737062889085
2	13	70	0	0	1737062905509
2	92	49	1	0	1737467607551
2	92	71	0	0	1737467618880
2	92	45	-1	0	1737467636818
2	92	57	-1	0	1737467641798
2	92	61	-1	0	1737467652960
9	6	9	-1	0	1739381449395
9	5	21	-1	0	1739383648005
9	5	25	-1	0	1739383674008
9	5	28	-1	0	1739383694906
12	10	4	-1	0	1739454677409
12	10	0	1	0	1739454683348
9	8	31	-1	0	1739454685545
12	15	4	-1	0	1739454690576
12	19	17	-1	0	1739454706652
12	21	0	-1	0	1739454720110
12	19	4	0	0	1739454722935
12	16	0	1	0	1739454724506
12	22	17	1	0	1739454726518
12	18	17	-1	0	1739454731772
12	21	17	0	0	1739454735231
12	25	17	1	0	1739454774992
12	25	0	-1	0	1739454848365
12	26	17	1	0	1739454857492
12	9	27	0	0	1739455350373
12	30	8	1	0	1739455740543
12	2	8	1	0	1739455743293
12	26	30	0	0	1739456260855
9	8	33	1	0	1739456794429
12	33	0	0	0	1739457886608
12	33	8	1	0	1739457894628
12	2	3	-1	0	1739458588965
12	2	32	-1	0	1739458598427
12	35	3	-1	0	1739459642307
12	13	13	-1	0	1739459677945
12	38	13	-1	0	1739459879364
12	17	23	-1	0	1739459880517
12	41	3	-1	0	1739459884131
12	39	32	-1	0	1739459885629
12	8	3	-1	0	1739460264775
12	44	0	1	0	1739460264830
12	47	6	-1	0	1739461833754
12	16	14	-1	0	1739462547087
12	54	17	0	0	1739462982566
12	5	20	-1	0	1739463844513
12	56	33	-1	0	1739464257100
12	15	8	-1	0	1739464276538
12	31	36	-1	0	1739464277282
12	34	18	-1	0	1739464997845
12	18	37	-1	0	1739465562853
12	31	21	-1	0	1739466301196
12	27	39	-1	0	1739466338136
12	2	41	0	0	1739468305715
12	60	4	-1	0	1739596178807
12	60	2	-1	0	1739596196032
12	60	39	0	0	1739596217337
2	23	12	-1	0	1736657307657
2	75	21	-1	0	1736740490617
2	75	24	-1	0	1736740496708
2	75	61	-1	0	1736740499685
2	75	57	0	0	1736740513412
4	0	3	0	0	1736755015553
5	1	2	1	0	1736794403226
5	8	6	-1	0	1736795633267
5	9	14	-1	0	1736795692908
5	9	6	-1	0	1736795718496
5	9	3	0	0	1736795728307
5	9	9	0	0	1736795963219
5	13	10	0	0	1736802906247
5	17	16	1	0	1736809743317
5	17	9	-1	0	1736809749733
5	17	15	-1	0	1736809764932
5	17	17	0	0	1736809774737
5	17	13	-1	0	1736809785813
5	17	19	0	0	1736809809680
2	78	48	0	0	1736827214740
2	78	60	-1	0	1736827230976
2	78	49	0	0	1736827700476
2	81	69	-1	0	1736879932364
2	81	40	0	0	1736879941077
2	81	33	-1	0	1736879949247
2	81	12	-1	0	1736879958198
2	84	2	-1	0	1736880731804
2	84	44	-1	0	1736880744272
2	84	3	-1	0	1736880749779
2	85	73	-1	0	1736881636142
2	85	63	-1	0	1736881645749
2	85	35	-1	0	1736881665546
2	85	54	-1	0	1736881671519
2	85	27	-1	0	1736881697042
2	85	17	-1	0	1736881723194
2	85	65	1	0	1736881749490
2	85	10	-1	0	1736881757282
2	85	68	-1	0	1736881769562
2	85	71	-1	0	1736881780882
2	85	51	1	0	1736881790161
2	85	64	1	0	1736881820982
2	85	28	-1	0	1736881835312
2	85	22	-1	0	1736881849565
2	87	2	-1	0	1736916090536
2	87	71	0	0	1736916607858
2	88	6	1	0	1736993944103
2	88	10	-1	0	1736993959471
2	88	17	-1	0	1736995023990
2	89	30	-1	0	1737002538005
2	90	54	-1	0	1737041528986
2	13	72	0	0	1737062909827
2	92	9	-1	0	1737467610464
2	92	48	1	0	1737467632016
2	92	52	-1	0	1737467639310
2	92	24	0	0	1737467664706
9	6	10	-1	0	1739381465646
9	5	8	0	0	1739384339208
12	14	17	1	0	1739454686963
12	11	0	1	0	1739454689164
12	15	17	-1	0	1739454694855
12	17	4	-1	0	1739454699301
12	9	28	0	0	1739455359909
12	8	28	1	0	1739455772822
12	10	28	1	0	1739455784769
12	8	19	-1	0	1739455785682
12	2	11	1	0	1739456284279
12	5	11	1	0	1739456304677
9	8	35	1	0	1739456798413
12	30	31	-1	0	1739457891204
12	2	33	-1	0	1739458623284
12	35	1	-1	0	1739459648841
12	35	2	-1	0	1739459658313
12	13	29	-1	0	1739459700555
12	41	30	1	0	1739459899087
12	39	29	1	0	1739459899689
12	8	7	-1	0	1739460277900
12	8	11	1	0	1739460281238
12	39	13	-1	0	1739461835179
12	39	0	0	0	1739461848284
12	49	13	-1	0	1739461878734
12	50	19	-1	0	1739461915051
12	49	7	-1	0	1739461915114
12	49	6	-1	0	1739461927513
12	49	31	-1	0	1739461930559
12	49	30	0	0	1739461938166
12	49	29	-1	0	1739461959238
12	16	25	-1	0	1739462551963
12	31	32	1	0	1739462985534
12	54	31	-1	0	1739462998304
12	54	33	-1	0	1739463020243
12	54	8	1	0	1739463061845
12	12	22	-1	0	1739463856612
12	56	20	-1	0	1739463951054
12	56	27	1	0	1739463974233
12	56	13	-1	0	1739463978753
12	35	8	-1	0	1739464303888
12	35	37	-1	0	1739465059282
12	35	21	1	0	1739465093929
12	12	21	0	0	1739465095634
12	18	18	-1	0	1739465571299
12	35	38	-1	0	1739466326645
12	27	38	0	0	1739466329159
12	2	40	-1	0	1739468310600
12	60	29	1	0	1739596234844
2	24	35	-1	0	1736657309082
2	20	29	-1	0	1736657311738
2	23	36	-1	0	1736657313749
2	24	29	-1	0	1736657315950
2	20	25	-1	0	1736657316580
2	23	3	-1	0	1736657317136
2	20	17	-1	0	1736657320458
2	24	25	-1	0	1736657321956
2	20	23	-1	0	1736657325164
2	20	24	-1	0	1736657330827
2	24	34	0	0	1736657331496
2	23	22	-1	0	1736657332086
2	20	11	-1	0	1736657337531
2	24	11	0	0	1736657338557
2	23	13	-1	0	1736657342211
2	24	31	-1	0	1736657344975
2	23	28	-1	0	1736657346008
2	23	27	-1	0	1736657351756
2	24	24	0	0	1736657352495
2	24	10	0	0	1736657357463
2	23	11	-1	0	1736657358443
2	23	25	-1	0	1736657362853
2	0	38	0	0	1736657364890
2	23	32	-1	0	1736657371773
2	24	12	0	0	1736657375384
2	23	38	-1	0	1736657380623
2	24	37	0	0	1736657382439
2	24	13	-1	0	1736657386559
2	23	5	-1	0	1736657389048
2	24	27	-1	0	1736657391665
2	24	3	-1	0	1736657395887
2	23	24	1	0	1736657396423
2	24	32	-1	0	1736657408551
2	24	38	1	0	1736657413556
2	24	22	0	0	1736657419514
2	24	9	0	0	1736657424242
2	24	17	-1	0	1736657428295
2	0	39	0	0	1736657461734
2	0	33	-1	0	1736657497967
2	0	35	-1	0	1736657501287
2	0	34	-1	0	1736657504401
2	15	36	0	0	1736657545419
2	15	37	0	0	1736657568236
2	15	39	-1	0	1736657575799
2	25	1	1	0	1736657594934
2	25	2	-1	0	1736657597129
2	25	6	1	0	1736657599291
2	25	28	-1	0	1736657603346
2	25	11	0	0	1736657610679
2	25	34	-1	0	1736657614086
2	25	39	-1	0	1736657617411
2	25	35	-1	0	1736657621357
2	25	30	-1	0	1736657625606
2	25	20	-1	0	1736657628072
2	25	29	-1	0	1736657632526
2	25	31	-1	0	1736657635237
2	25	22	-1	0	1736657639293
2	25	36	-1	0	1736657641297
2	25	5	-1	0	1736657647507
2	25	13	-1	0	1736657649209
2	25	12	-1	0	1736657651255
2	25	33	-1	0	1736657654088
2	25	10	-1	0	1736657656245
2	25	37	-1	0	1736657661957
2	25	32	-1	0	1736657682928
2	25	27	-1	0	1736657686467
2	25	21	-1	0	1736657689104
2	25	23	-1	0	1736657691391
2	25	17	-1	0	1736657693114
2	25	15	-1	0	1736657695146
2	25	38	-1	0	1736657696799
2	25	9	-1	0	1736657699090
2	25	25	-1	0	1736657700905
2	25	3	-1	0	1736657703591
2	25	24	1	0	1736657707646
2	26	31	-1	0	1736657746112
2	26	1	0	0	1736657766973
2	26	2	0	0	1736657771384
2	26	30	-1	0	1736657784574
2	27	38	1	0	1736657793193
2	26	29	-1	0	1736657801372
2	26	6	-1	0	1736657803271
2	27	31	0	0	1736657806065
2	27	1	1	0	1736657808318
2	26	15	-1	0	1736657813040
2	27	6	1	0	1736657814631
2	26	36	-1	0	1736657817140
2	27	2	1	0	1736657818573
2	27	9	-1	0	1736657828001
2	28	6	1	0	1736657847075
2	27	39	1	0	1736657847975
2	28	1	1	0	1736657849635
2	27	27	-1	0	1736657856810
2	27	30	1	0	1736657861762
2	28	2	1	0	1736657863255
2	27	10	-1	0	1736657865012
2	28	20	-1	0	1736657866560
2	28	30	1	0	1736657868385
2	27	5	-1	0	1736657869336
2	28	3	-1	0	1736657870892
2	28	27	-1	0	1736657875700
2	28	38	1	0	1736657878843
2	27	35	-1	0	1736657881564
2	28	39	1	0	1736657881647
2	28	37	-1	0	1736657884052
2	27	3	-1	0	1736657887848
2	28	25	0	0	1736657889534
2	28	9	0	0	1736657891664
2	27	22	1	0	1736657892930
2	28	22	0	0	1736657898042
2	28	31	1	0	1736657901013
2	27	36	-1	0	1736657901941
2	28	10	-1	0	1736657903120
2	28	33	1	0	1736657906289
2	27	13	1	0	1736657908722
2	28	35	1	0	1736657909043
2	28	29	-1	0	1736657911988
2	28	13	0	0	1736657914623
2	28	34	1	0	1736657918241
2	28	23	-1	0	1736657920594
2	27	34	1	0	1736657921769
2	28	32	1	0	1736657924985
2	29	1	1	0	1736657926210
2	28	15	0	0	1736657928724
2	27	33	1	0	1736657932688
2	28	24	1	0	1736657932850
2	28	17	-1	0	1736657934604
2	28	36	-1	0	1736657936439
2	29	34	-1	0	1736657937270
2	28	28	1	0	1736657938160
2	27	23	-1	0	1736657938943
2	28	21	-1	0	1736657944297
2	27	32	-1	0	1736657946595
2	28	12	1	0	1736657946596
2	29	39	-1	0	1736657946823
2	28	5	-1	0	1736657948345
2	29	6	-1	0	1736657949306
2	28	11	0	0	1736657950789
2	29	2	1	0	1736657965049
2	27	24	1	0	1736657967905
2	29	38	-1	0	1736657971530
2	27	28	-1	0	1736657974368
2	29	27	0	0	1736657987126
2	27	37	-1	0	1736657989119
2	29	35	-1	0	1736657992436
2	27	25	-1	0	1736657994401
2	27	20	-1	0	1736657999131
2	29	5	0	0	1736658001609
2	27	21	-1	0	1736658004110
2	27	12	0	0	1736658010729
2	27	15	-1	0	1736658013902
2	27	17	-1	0	1736658016782
2	29	36	-1	0	1736658018941
2	27	11	-1	0	1736658019841
2	29	20	0	0	1736658026587
2	27	29	-1	0	1736658029912
2	29	30	-1	0	1736658035857
2	29	33	-1	0	1736658046942
2	76	6	1	0	1736742667051
2	76	30	0	0	1736742683759
2	76	42	-1	0	1736742696841
2	76	66	-1	0	1736742711374
2	76	70	-1	0	1736742722354
2	76	43	-1	0	1736742726372
2	76	31	-1	0	1736742740725
2	76	21	-1	0	1736742750557
2	76	25	-1	0	1736742757752
2	76	54	-1	0	1736742780824
2	76	35	-1	0	1736742787827
2	76	48	0	0	1736742809892
4	0	4	0	0	1736755039857
5	4	13	-1	0	1736794490397
5	3	7	-1	0	1736794501992
5	3	5	0	0	1736794521629
5	3	12	1	0	1736794542524
5	9	8	-1	0	1736795948713
5	9	10	0	0	1736795951600
5	9	12	0	0	1736795960464
5	13	1	0	0	1736802932675
5	13	9	-1	0	1736802939009
5	13	2	0	0	1736802941636
5	17	10	1	0	1736809745526
5	17	22	-1	0	1736809755395
5	17	3	1	0	1736809759920
2	78	72	0	0	1736827220716
2	78	33	-1	0	1736827244101
2	78	69	1	0	1736827741625
2	78	32	-1	0	1736827747930
2	78	20	-1	0	1736827762376
2	81	70	-1	0	1736879935585
2	81	30	-1	0	1736879947566
2	81	61	0	0	1736879954853
2	82	44	0	0	1736880734490
2	84	45	-1	0	1736880737308
2	84	1	0	0	1736880741479
2	84	6	-1	0	1736880747018
2	84	20	-1	0	1736880752552
2	82	51	1	0	1736880756854
2	84	53	-1	0	1736880757302
2	85	25	-1	0	1736881701125
2	85	56	-1	0	1736881710146
2	85	3	-1	0	1736881715942
2	87	6	-1	0	1736916093680
2	87	22	0	0	1736916615920
2	88	2	-1	0	1736993950800
2	88	37	-1	0	1736996045346
2	89	46	-1	0	1737002556626
2	89	56	-1	0	1737002579571
2	90	28	-1	0	1737041560321
2	90	34	-1	0	1737041576953
2	90	51	1	0	1737041584814
6	0	0	0	0	1737084575710
2	92	50	1	0	1737467647379
11	0	0	0	0	1739296894842
11	9	2	-1	0	1739302658326
9	0	0	-1	0	1739317205304
9	6	11	-1	0	1739381475543
9	7	1	0	0	1739384417629
12	18	4	-1	0	1739454721010
12	24	4	-1	0	1739454751640
12	23	17	-1	0	1739454751871
12	9	19	-1	0	1739455374017
12	30	0	1	0	1739455776765
12	8	29	1	0	1739455780401
12	5	29	1	0	1739456284557
12	5	6	-1	0	1739456308200
9	8	32	1	0	1739456812426
12	30	7	1	0	1739457907548
12	18	32	0	0	1739458634363
12	18	3	-1	0	1739458636826
12	18	2	-1	0	1739458638831
12	5	33	-1	0	1739458676648
12	4	33	-1	0	1739458678357
12	12	33	-1	0	1739458699998
12	36	28	1	0	1739459655480
12	35	29	0	0	1739459668189
12	14	31	-1	0	1739459900246
12	30	1	-1	0	1739459901329
12	40	23	-1	0	1739459904102
12	35	34	-1	0	1739459904299
12	45	30	0	0	1739460441695
12	47	27	-1	0	1739461837424
12	47	0	-1	0	1739461838815
12	2	14	-1	0	1739462574247
12	54	29	-1	0	1739463025667
12	54	30	1	0	1739463034984
12	12	20	-1	0	1739463862455
12	15	30	-1	0	1739464309884
12	27	18	-1	0	1739465061195
12	27	25	-1	0	1739465063934
12	27	37	-1	0	1739465091117
12	3	18	-1	0	1739465091378
12	20	21	1	0	1739465091920
12	27	20	-1	0	1739465095040
12	12	37	-1	0	1739465105941
12	20	37	-1	0	1739465107065
12	53	30	1	0	1739465142047
12	53	37	-1	0	1739465177973
12	3	20	-1	0	1739465240321
12	34	26	-1	0	1739465646357
12	35	39	1	0	1739466332679
12	3	41	-1	0	1739468503109
12	60	7	0	0	1739596241263
2	29	31	-1	0	1736658051786
2	29	37	-1	0	1736658065317
2	29	17	-1	0	1736658067963
2	29	32	-1	0	1736658084660
2	0	40	0	0	1736658087582
2	29	13	-1	0	1736658087793
2	29	25	-1	0	1736658093021
2	29	40	-1	0	1736658099800
2	29	12	-1	0	1736658106570
2	76	2	0	0	1736742672659
2	76	9	-1	0	1736742679776
2	76	29	0	0	1736742691698
2	76	45	-1	0	1736742699861
2	76	69	0	0	1736742706873
2	76	72	-1	0	1736742716267
2	76	34	-1	0	1736742736341
2	76	27	-1	0	1736742747769
2	76	3	-1	0	1736742755637
2	76	55	0	0	1736742768193
2	76	22	0	0	1736742774532
2	76	23	-1	0	1736742783527
2	76	5	0	0	1736742825118
2	76	40	-1	0	1736742833261
2	76	33	-1	0	1736742843420
2	76	57	0	0	1736742863164
2	76	56	0	0	1736742877178
2	76	65	-1	0	1736742890180
2	76	28	0	0	1736742916244
2	76	52	0	0	1736742944042
2	76	49	0	0	1736742951575
2	76	11	-1	0	1736742958747
4	1	1	-1	0	1736756067769
5	3	3	1	0	1736794510052
5	3	10	-1	0	1736794523645
5	3	13	1	0	1736794530716
5	3	9	-1	0	1736794533565
5	3	8	-1	0	1736794541037
5	9	16	0	0	1736795950446
5	9	17	0	0	1736795959240
5	9	2	0	0	1736795961694
5	13	11	0	0	1736802934387
5	17	5	-1	0	1736809748007
5	17	12	1	0	1736809751367
5	17	2	1	0	1736809757541
5	17	11	1	0	1736809766393
5	17	14	-1	0	1736809804073
2	78	66	-1	0	1736827227105
2	78	13	-1	0	1736827266222
2	78	24	0	0	1736827924996
2	82	6	-1	0	1736880375756
2	82	11	0	0	1736880767259
2	82	55	-1	0	1736880771462
2	85	40	-1	0	1736881726293
2	85	41	-1	0	1736881732992
2	85	31	-1	0	1736881739322
2	85	43	-1	0	1736881755423
2	85	20	-1	0	1736881764244
2	85	15	-1	0	1736881783621
2	85	46	-1	0	1736881786829
2	85	72	0	0	1736881805410
2	85	60	-1	0	1736881815948
2	85	12	-1	0	1736881823890
2	85	45	-1	0	1736881832511
2	85	59	-1	0	1736881839492
2	85	52	1	0	1736881858482
2	85	37	0	0	1736881869800
2	85	42	-1	0	1736881879566
2	85	36	-1	0	1736881885131
2	87	21	-1	0	1736916102414
2	87	66	-1	0	1736916632372
2	88	23	-1	0	1736994585304
2	88	24	1	0	1736995391630
2	89	21	-1	0	1737002559063
2	89	12	-1	0	1737002561814
2	90	45	-1	0	1737041569197
6	0	1	-1	0	1737096160075
2	92	62	-1	0	1737467674727
11	0	1	0	0	1739296903352
11	10	0	1	0	1739308139115
9	1	1	-1	0	1739317231804
9	6	12	-1	0	1739381490697
9	8	29	-1	0	1739453811408
12	23	0	1	0	1739454744889
9	11	31	0	0	1739455389412
12	10	27	1	0	1739455781164
12	8	27	1	0	1739455789778
12	8	8	-1	0	1739455811603
12	10	29	1	0	1739455823394
9	11	33	-1	0	1739456366347
12	20	11	-1	0	1739456435124
12	12	27	1	0	1739456437133
12	4	11	0	0	1739456437804
12	4	30	0	0	1739456444595
12	22	30	0	0	1739456473555
12	22	6	-1	0	1739456501526
12	32	6	-1	0	1739456504788
9	11	36	-1	0	1739456937285
12	0	32	0	0	1739457992791
12	18	33	-1	0	1739458642202
12	34	0	-1	0	1739458655658
12	12	3	-1	0	1739458704175
12	36	31	-1	0	1739459661649
12	13	3	-1	0	1739459670851
12	35	33	-1	0	1739459678617
12	36	3	-1	0	1739459690777
12	37	2	-1	0	1739459707421
12	7	27	-1	0	1739459910233
12	30	13	-1	0	1739459911798
12	43	34	-1	0	1739459913119
12	43	33	-1	0	1739459915812
12	14	23	-1	0	1739459921696
12	43	3	-1	0	1739459926002
12	17	34	0	0	1739459927256
12	43	17	-1	0	1739459928457
12	17	28	0	0	1739459955201
12	43	0	-1	0	1739459962382
12	41	4	-1	0	1739459966338
12	43	27	1	0	1739459976462
12	40	7	1	0	1739460019770
12	16	8	0	0	1739460030723
12	40	11	1	0	1739460032935
12	40	28	1	0	1739460039485
12	27	3	-1	0	1739460058403
12	45	1	0	0	1739460486771
12	45	13	-1	0	1739460538421
12	12	13	-1	0	1739461839197
12	12	34	-1	0	1739461844003
12	3	13	-1	0	1739461850982
12	39	19	-1	0	1739461854906
12	3	3	-1	0	1739461857102
12	39	23	-1	0	1739461860375
12	3	33	-1	0	1739461863230
12	2	25	-1	0	1739462578014
12	12	14	-1	0	1739463042685
12	12	25	-1	0	1739463046134
12	12	35	-1	0	1739463865914
12	15	28	1	0	1739464320692
12	27	22	-1	0	1739465068838
12	30	18	-1	0	1739465083732
12	12	26	-1	0	1739465718391
12	27	26	-1	0	1739466333039
12	17	18	-1	0	1739466368645
12	18	40	-1	0	1739480177392
12	18	38	-1	0	1739480200593
12	60	30	1	0	1739596281297
12	60	32	1	0	1739596295734
2	29	9	-1	0	1736658102325
2	29	23	0	0	1736658115527
2	29	29	-1	0	1736658125951
2	29	15	-1	0	1736658132097
2	29	28	-1	0	1736658137107
2	29	3	-1	0	1736658145531
2	29	11	-1	0	1736658148333
2	29	10	-1	0	1736658152626
2	29	21	-1	0	1736658159482
2	30	6	1	0	1736658160280
2	29	22	-1	0	1736658163349
2	30	2	1	0	1736658165664
2	0	41	0	0	1736658171671
2	29	24	1	0	1736658172197
2	30	34	-1	0	1736658174440
2	30	38	-1	0	1736658180293
2	30	17	-1	0	1736658183478
2	30	1	1	0	1736658185367
2	29	41	0	0	1736658190483
2	30	39	-1	0	1736658190596
2	30	35	-1	0	1736658193926
2	30	30	0	0	1736658197308
2	30	11	0	0	1736658199861
2	30	28	-1	0	1736658202891
2	30	40	-1	0	1736658208103
2	30	33	-1	0	1736658211047
2	30	36	-1	0	1736658213441
2	30	20	-1	0	1736658214808
2	30	31	-1	0	1736658218090
2	30	25	-1	0	1736658219991
2	30	12	-1	0	1736658222546
2	30	27	-1	0	1736658226070
2	30	21	-1	0	1736658228060
2	30	9	-1	0	1736658229930
2	30	13	-1	0	1736658232605
2	30	23	-1	0	1736658235715
2	30	32	-1	0	1736658240431
2	30	10	-1	0	1736658242213
2	30	3	-1	0	1736658244014
2	30	5	-1	0	1736658245424
2	30	15	-1	0	1736658247160
2	30	37	-1	0	1736658252131
2	30	29	-1	0	1736658256068
2	30	41	0	0	1736658264025
2	30	22	0	0	1736658267876
2	30	24	0	0	1736658275014
2	15	38	0	0	1736658725415
2	15	40	-1	0	1736658733478
2	0	42	-1	0	1736659014858
2	0	43	-1	0	1736659124753
2	0	44	-1	0	1736659147408
2	0	45	0	0	1736659968264
2	13	42	-1	0	1736659995878
2	13	39	-1	0	1736660000997
2	13	43	-1	0	1736660002921
2	13	44	-1	0	1736660004344
2	13	40	-1	0	1736660007582
2	13	38	-1	0	1736660009645
2	13	45	-1	0	1736660016097
2	13	41	-1	0	1736660019536
2	13	46	-1	0	1736660283934
2	0	47	0	0	1736661684719
2	31	6	1	0	1736663252764
2	31	43	-1	0	1736663266396
2	31	2	1	0	1736663274182
2	31	46	-1	0	1736663277125
2	31	40	-1	0	1736663280415
2	31	30	-1	0	1736663284153
2	31	13	-1	0	1736663293480
2	31	38	-1	0	1736663295339
2	31	1	1	0	1736663297379
2	31	39	-1	0	1736663309188
2	31	45	-1	0	1736663312871
2	31	36	-1	0	1736663316891
2	31	10	-1	0	1736663318828
2	31	34	-1	0	1736663327577
2	31	42	-1	0	1736663332912
2	31	23	-1	0	1736663340178
2	31	33	-1	0	1736663347128
2	31	44	0	0	1736663352921
2	31	35	-1	0	1736663356152
2	31	47	-1	0	1736663365528
2	31	32	0	0	1736663379016
2	31	22	1	0	1736663382186
2	31	31	0	0	1736663399243
2	31	15	-1	0	1736663401836
2	31	3	-1	0	1736663404244
2	31	27	-1	0	1736663408192
2	31	9	-1	0	1736663410136
2	31	12	-1	0	1736663420329
2	31	28	-1	0	1736663423953
2	31	21	-1	0	1736663428402
2	31	5	-1	0	1736663432999
2	31	11	1	0	1736663441261
2	31	25	-1	0	1736663443948
2	31	41	1	0	1736663461088
2	31	17	-1	0	1736663463493
2	31	37	-1	0	1736663466957
2	31	29	1	0	1736663476557
2	31	24	-1	0	1736663480953
2	31	20	-1	0	1736663487786
2	31	48	-1	0	1736663636885
2	31	49	-1	0	1736663822231
2	31	50	-1	0	1736663834228
2	32	30	-1	0	1736664442727
2	32	40	-1	0	1736664446565
2	32	42	-1	0	1736664450392
2	32	2	-1	0	1736664453483
2	32	49	-1	0	1736664458125
2	32	45	-1	0	1736664464430
2	32	6	1	0	1736664468487
2	32	50	-1	0	1736664471957
2	32	39	-1	0	1736664477169
2	32	38	-1	0	1736664479309
2	32	9	-1	0	1736664481548
2	32	43	-1	0	1736664483832
2	32	1	1	0	1736664487442
2	32	35	-1	0	1736664489850
2	32	48	-1	0	1736664496265
2	32	3	-1	0	1736664498911
2	32	46	-1	0	1736664502771
2	32	34	-1	0	1736664506411
2	32	31	-1	0	1736664510110
2	32	24	0	0	1736664640488
2	32	47	0	0	1736664645147
2	32	10	-1	0	1736664647636
2	32	22	0	0	1736664651530
2	32	36	-1	0	1736664654970
2	32	20	0	0	1736664658596
2	32	33	-1	0	1736664661568
2	32	27	-1	0	1736664664736
2	32	11	0	0	1736664667481
2	32	28	-1	0	1736664671966
2	32	13	-1	0	1736664676805
2	32	29	-1	0	1736664681068
2	32	44	-1	0	1736664683353
2	32	32	0	0	1736664706715
2	32	12	-1	0	1736664709555
2	32	25	-1	0	1736664713406
2	32	41	0	0	1736664719245
2	32	15	-1	0	1736664721527
2	32	5	-1	0	1736664723389
2	32	17	-1	0	1736664725471
2	32	37	0	0	1736664730227
2	32	21	-1	0	1736664733198
2	32	23	0	0	1736664737456
2	33	46	-1	0	1736666020919
2	33	1	1	0	1736666023538
2	33	48	-1	0	1736666034383
2	33	30	-1	0	1736666039316
2	33	6	1	0	1736666041219
2	33	2	1	0	1736666043423
2	33	27	-1	0	1736666049648
2	33	42	-1	0	1736666055588
2	33	45	-1	0	1736666061337
2	33	49	1	0	1736666067664
2	33	31	-1	0	1736666072881
2	33	33	1	0	1736666081095
2	33	43	0	0	1736666084166
2	33	47	-1	0	1736666096096
2	33	9	-1	0	1736666102034
2	33	13	-1	0	1736666109370
2	33	24	1	0	1736666114142
2	33	34	-1	0	1736666134556
2	76	1	1	0	1736742675908
2	76	67	-1	0	1736742718231
2	76	20	-1	0	1736742724729
2	76	71	-1	0	1736742730972
2	76	13	-1	0	1736742753444
2	76	44	-1	0	1736742759951
2	76	51	0	0	1736742777747
2	76	39	-1	0	1736742797095
4	0	5	0	0	1736756088846
4	0	6	0	0	1736756098054
4	1	6	-1	0	1736756110718
5	1	14	-1	0	1736794575388
5	4	15	-1	0	1736794601770
5	8	18	-1	0	1736796013775
5	14	18	-1	0	1736803832877
5	17	6	-1	0	1736809792308
5	17	7	-1	0	1736809796682
5	17	21	-1	0	1736809811710
2	78	68	0	0	1736827254403
2	78	31	-1	0	1736827261276
2	78	41	-1	0	1736827953153
2	82	2	-1	0	1736880384219
2	82	33	1	0	1736880406151
2	82	52	0	0	1736880785062
2	85	13	-1	0	1736881753240
2	85	23	-1	0	1736881762294
2	85	33	-1	0	1736881775054
2	85	47	-1	0	1736881798512
2	85	48	0	0	1736881811662
2	85	58	-1	0	1736881827324
2	85	66	-1	0	1736881844383
2	87	68	0	0	1736916124243
2	87	23	0	0	1736916147278
2	87	11	0	0	1736916639976
2	87	64	-1	0	1736916660383
2	88	33	-1	0	1736994596651
2	88	63	-1	0	1736996049537
2	89	42	-1	0	1737002585230
2	89	64	-1	0	1737002598980
2	90	20	-1	0	1737041596277
2	91	1	1	0	1737265238157
2	91	2	-1	0	1737265244076
3	7	7	-1	0	1737730363690
11	1	1	-1	0	1739296921442
11	10	2	-1	0	1739308140927
11	13	10	-1	0	1739318781200
12	0	0	-1	0	1739381900528
9	9	29	0	0	1739453994102
12	22	4	0	0	1739454779915
12	19	0	1	0	1739454789126
9	11	29	0	0	1739455395543
12	8	23	-1	0	1739455816146
12	11	11	-1	0	1739456382376
12	12	23	-1	0	1739456415238
9	8	36	1	0	1739456947959
12	5	32	1	0	1739458081709
12	34	27	1	0	1739458671156
12	5	3	-1	0	1739458681527
12	36	13	0	0	1739459668992
12	7	8	1	0	1739459916530
12	14	1	-1	0	1739459917088
12	43	1	-1	0	1739459923783
12	14	28	1	0	1739459933883
12	17	7	0	0	1739459941718
12	16	30	1	0	1739459943602
12	45	3	0	0	1739460555114
12	39	27	1	0	1739461840232
12	47	4	-1	0	1739461841272
12	3	2	-1	0	1739461854209
12	39	1	-1	0	1739461865189
12	3	1	-1	0	1739461869405
12	3	34	-1	0	1739461873249
12	48	31	-1	0	1739461875400
9	14	34	-1	0	1739462748009
12	54	32	-1	0	1739463054926
12	56	22	-1	0	1739463945527
12	56	2	-1	0	1739463963264
12	29	8	1	0	1739464333298
12	3	36	-1	0	1739465070627
12	3	21	-1	0	1739465078792
12	0	38	0	0	1739465889801
12	41	18	-1	0	1739465944293
12	5	26	-1	0	1739465949668
12	49	21	1	0	1739465960613
12	35	26	-1	0	1739466347043
12	18	39	-1	0	1739480183366
12	60	8	1	0	1739596286725
12	60	17	-1	0	1739596300565
2	33	28	-1	0	1736666088830
2	33	17	-1	0	1736666117638
2	76	38	0	0	1736742819178
2	76	63	0	0	1736742831321
2	76	10	-1	0	1736742835084
2	76	58	0	0	1736742854040
2	76	47	0	0	1736742871094
2	76	53	1	0	1736742887183
2	76	37	-1	0	1736742905573
2	76	46	-1	0	1736742925279
2	76	32	0	0	1736742936427
4	1	0	1	0	1736756098359
4	1	3	-1	0	1736756104298
5	1	13	-1	0	1736794584826
5	8	19	-1	0	1736796202086
5	14	20	-1	0	1736803850477
5	14	9	1	0	1736803866150
5	17	8	-1	0	1736809821847
5	17	20	-1	0	1736809824492
5	17	18	-1	0	1736809828973
2	78	43	-1	0	1736827256734
2	78	12	-1	0	1736827263524
2	78	38	1	0	1736827272022
2	78	57	-1	0	1736827957674
2	82	64	-1	0	1736880391530
2	82	67	-1	0	1736880789282
2	85	44	-1	0	1736881852431
2	85	57	0	0	1736881873551
2	85	53	-1	0	1736881881864
2	87	17	-1	0	1736916127003
2	87	46	-1	0	1736916131997
2	87	54	-1	0	1736916645354
2	88	70	-1	0	1736994600266
2	88	61	-1	0	1736996051820
2	89	15	-1	0	1737002589051
2	89	51	1	0	1737002602947
2	90	46	0	0	1737041636696
2	90	48	-1	0	1737041646761
2	91	70	-1	0	1737265252354
2	91	68	-1	0	1737265282891
2	91	65	-1	0	1737265292194
2	91	61	0	0	1737265304663
2	91	63	-1	0	1737265322262
2	91	35	-1	0	1737265335327
2	91	56	0	0	1737265351328
3	7	8	-1	0	1737730617977
11	1	0	-1	0	1739296923122
11	4	1	-1	0	1739296950850
11	6	0	0	0	1739296969754
11	10	3	-1	0	1739308174193
9	2	1	-1	0	1739345825869
12	0	1	-1	0	1739381958832
9	9	30	-1	0	1739454039938
12	14	0	0	0	1739454843930
9	11	30	0	0	1739455397455
12	10	8	1	0	1739455828884
12	5	8	1	0	1739456402079
9	8	37	-1	0	1739457013474
12	12	7	-1	0	1739458094051
12	21	32	1	0	1739458107533
12	21	11	1	0	1739458119855
12	34	30	1	0	1739458134536
12	5	2	-1	0	1739458679741
12	30	3	-1	0	1739458688090
12	12	2	-1	0	1739458707983
12	0	34	0	0	1739458745061
12	36	33	-1	0	1739459674187
12	36	7	0	0	1739459687947
12	13	31	0	0	1739459694176
12	41	17	1	0	1739459952778
12	16	6	-1	0	1739459954957
12	43	19	-1	0	1739459957084
12	43	11	1	0	1739459960139
12	16	31	-1	0	1739459960602
12	43	4	1	0	1739459964230
9	13	36	-1	0	1739460658392
9	13	33	-1	0	1739460669250
12	39	3	-1	0	1739461868156
9	14	37	-1	0	1739462751613
12	0	35	0	0	1739463187100
12	56	17	0	0	1739463960186
12	57	11	1	0	1739464340721
12	56	8	1	0	1739464353243
12	22	18	-1	0	1739465070629
12	15	38	-1	0	1739465930480
12	15	26	-1	0	1739465938177
12	41	26	-1	0	1739465938731
12	22	20	-1	0	1739465940594
12	5	38	-1	0	1739465952105
12	22	25	-1	0	1739465960908
12	22	14	-1	0	1739465967584
12	59	0	-1	0	1739465969716
12	59	32	0	0	1739465985280
12	59	22	-1	0	1739466006679
12	59	18	-1	0	1739466011495
12	59	34	1	0	1739466026443
12	59	1	-1	0	1739466029077
12	15	39	-1	0	1739466030030
12	59	20	-1	0	1739466031969
12	17	26	-1	0	1739466374031
12	18	41	-1	0	1739480191932
12	60	11	1	0	1739596330075
2	33	40	-1	0	1736666099490
2	33	44	-1	0	1736666105515
2	33	39	-1	0	1736666123399
2	33	32	-1	0	1736666129845
2	33	37	-1	0	1736666139335
2	33	3	-1	0	1736666142553
2	33	50	-1	0	1736666146534
2	33	38	-1	0	1736666148469
2	33	22	1	0	1736666153459
2	33	35	-1	0	1736666160901
2	33	21	-1	0	1736666165124
2	33	20	-1	0	1736666167472
2	33	10	-1	0	1736666171058
2	33	29	-1	0	1736666181014
2	33	25	-1	0	1736666185801
2	33	12	-1	0	1736666188159
2	33	36	-1	0	1736666191731
2	33	5	-1	0	1736666194458
2	33	23	-1	0	1736666197771
2	33	11	1	0	1736666201000
2	33	15	-1	0	1736666204062
2	33	41	-1	0	1736666211718
2	0	51	0	0	1736666547058
2	0	50	0	0	1736666757483
2	0	46	-1	0	1736666760591
2	0	48	0	0	1736666782162
2	0	49	0	0	1736666784315
2	0	52	0	0	1736666930825
2	34	53	-1	0	1736669336139
2	35	6	0	0	1736673036880
2	35	12	0	0	1736673042964
2	35	30	0	0	1736673047231
2	11	30	-1	0	1736676176237
2	11	39	-1	0	1736676181649
2	11	42	-1	0	1736676185688
2	11	49	1	0	1736676193058
2	11	43	-1	0	1736676195638
2	11	47	-1	0	1736676201920
2	11	45	-1	0	1736676210110
2	11	38	-1	0	1736676213568
2	11	37	-1	0	1736676226398
2	11	44	1	0	1736676231887
2	11	34	-1	0	1736676240378
2	11	48	1	0	1736676258597
2	11	31	-1	0	1736676262407
2	11	35	-1	0	1736676265798
2	11	54	-1	0	1736676325380
2	11	55	-1	0	1736676364798
2	11	50	-1	0	1736676695159
2	11	46	-1	0	1736676701509
2	11	40	-1	0	1736676705760
2	11	28	-1	0	1736676711239
2	11	33	-1	0	1736676724738
2	11	53	-1	0	1736676728508
2	11	36	-1	0	1736676737197
2	11	41	1	0	1736676767837
2	11	32	-1	0	1736676779137
2	11	52	-1	0	1736676783017
2	11	51	-1	0	1736676787287
2	11	29	-1	0	1736676792747
2	36	6	1	0	1736678082209
2	36	55	-1	0	1736678089339
2	36	46	-1	0	1736678100297
2	36	30	0	0	1736678106797
2	36	20	-1	0	1736678112727
2	36	42	-1	0	1736678117837
2	36	2	1	0	1736678124837
2	36	35	-1	0	1736678129880
2	36	48	-1	0	1736678137238
2	36	49	1	0	1736678151723
2	36	45	1	0	1736678163877
2	36	54	-1	0	1736678168932
2	36	1	1	0	1736678173087
2	36	53	-1	0	1736678176817
2	36	44	-1	0	1736678181958
2	36	47	-1	0	1736678192537
2	36	52	0	0	1736678199137
2	36	21	1	0	1736678205481
2	36	34	-1	0	1736678212537
2	36	50	0	0	1736678222057
2	36	38	1	0	1736678226017
2	36	36	-1	0	1736678231958
2	36	15	-1	0	1736678236767
2	36	39	1	0	1736678241938
2	36	22	0	0	1736678250052
2	36	13	-1	0	1736678255078
2	36	40	-1	0	1736678257897
2	36	31	-1	0	1736678264672
2	36	51	-1	0	1736678267462
2	36	33	-1	0	1736678273318
2	36	43	-1	0	1736678276007
2	36	5	-1	0	1736678279237
2	36	24	1	0	1736678287077
2	36	29	0	0	1736678295922
2	36	17	-1	0	1736678300117
2	36	27	-1	0	1736678308362
2	36	25	-1	0	1736678312819
2	36	12	-1	0	1736678317497
2	36	32	-1	0	1736678321397
2	36	9	-1	0	1736678329498
2	36	23	-1	0	1736678334862
2	36	37	-1	0	1736678341618
2	36	41	-1	0	1736678350532
2	36	3	-1	0	1736678354332
2	36	11	0	0	1736678360127
2	36	10	-1	0	1736678365702
2	36	28	-1	0	1736678370069
2	37	56	-1	0	1736679614051
2	38	32	-1	0	1736690333427
2	38	53	-1	0	1736690337777
2	38	30	0	0	1736690344946
2	38	6	1	0	1736690349590
2	38	34	-1	0	1736690357122
2	38	55	-1	0	1736690362237
2	38	48	0	0	1736690372673
2	38	2	0	0	1736690380643
2	38	54	-1	0	1736690384606
2	38	49	-1	0	1736690396367
2	38	43	-1	0	1736690399471
2	38	39	-1	0	1736690533210
2	38	1	1	0	1736690536680
2	38	42	-1	0	1736690542608
2	38	33	-1	0	1736690550013
2	39	30	1	0	1736696208703
2	39	55	-1	0	1736696215175
2	39	6	-1	0	1736696219543
2	39	10	-1	0	1736696222413
2	39	50	0	0	1736696227733
2	39	1	1	0	1736696229892
2	39	42	-1	0	1736696233312
2	39	2	0	0	1736696237268
2	39	49	0	0	1736696241687
2	39	38	-1	0	1736696247275
2	39	36	-1	0	1736696251148
2	39	44	-1	0	1736696253668
2	39	37	0	0	1736696258350
2	39	28	-1	0	1736696262689
2	11	56	-1	0	1736699155169
2	40	39	-1	0	1736699354428
2	13	55	-1	0	1736699358544
2	13	54	-1	0	1736699360193
2	13	53	-1	0	1736699362484
2	13	51	-1	0	1736699364764
2	13	52	-1	0	1736699369007
2	13	56	-1	0	1736699388017
2	13	49	0	0	1736699396493
2	13	48	0	0	1736699409535
2	13	50	0	0	1736699411420
2	13	47	-1	0	1736699429213
2	41	2	0	0	1736699599450
2	41	39	-1	0	1736699603864
2	41	55	-1	0	1736699607980
2	41	53	0	0	1736699614971
2	41	1	0	0	1736699626004
2	41	43	0	0	1736699630413
2	41	51	0	0	1736699636916
2	41	30	0	0	1736699641333
2	41	6	1	0	1736699649109
2	76	36	-1	0	1736742857933
2	76	12	-1	0	1736742859792
2	76	15	-1	0	1736742865200
2	76	68	-1	0	1736742897838
2	76	64	-1	0	1736742908001
4	1	5	-1	0	1736756101127
4	1	4	-1	0	1736756107372
4	1	2	-1	0	1736756116183
5	1	15	-1	0	1736794607627
5	9	19	0	0	1736796245486
5	8	20	-1	0	1736796439132
5	14	17	-1	0	1736803856554
5	14	13	1	0	1736803879618
5	14	5	1	0	1736803911322
5	14	15	-1	0	1736803926819
5	14	16	1	0	1736803937247
5	14	6	1	0	1736803946723
5	14	2	1	0	1736803949900
5	17	4	-1	0	1736809832972
5	8	21	-1	0	1736809834956
5	8	22	-1	0	1736809839089
2	78	27	-1	0	1736827385606
2	11	73	-1	0	1736879439645
2	11	60	-1	0	1736879449634
2	11	59	0	0	1736879500251
2	82	21	0	0	1736880437861
2	82	42	-1	0	1736880449540
2	83	13	-1	0	1736880480014
2	83	30	0	0	1736880487035
2	82	59	1	0	1736880816031
2	85	5	1	0	1736881888981
2	85	11	-1	0	1736881902982
2	85	62	0	0	1736881923472
2	85	49	-1	0	1736881926280
2	87	63	0	0	1736916158887
2	87	42	-1	0	1736916664061
2	88	30	-1	0	1736994623410
2	89	72	1	0	1737002015165
2	90	1	1	0	1737041230373
2	90	12	-1	0	1737041650801
2	91	34	-1	0	1737265258365
2	91	6	-1	0	1737265264085
2	91	30	-1	0	1737265278330
2	91	54	-1	0	1737265299052
2	91	23	-1	0	1737265315969
2	91	73	0	0	1737265332884
2	91	60	-1	0	1737265338269
3	7	4	1	0	1737730619993
11	2	0	1	0	1739296933290
11	3	1	1	0	1739296943378
11	4	0	1	0	1739296950363
11	5	0	0	0	1739296960293
11	10	4	-1	0	1739308313962
9	2	0	-1	0	1739345831839
12	0	2	-1	0	1739382030967
12	0	3	-1	0	1739382037877
12	0	4	-1	0	1739382045268
12	0	10	-1	0	1739382097654
12	2	0	1	0	1739454339184
12	26	4	-1	0	1739454852991
12	26	0	-1	0	1739454865165
12	27	17	-1	0	1739454891768
12	20	27	-1	0	1739455517757
12	20	29	-1	0	1739455835475
12	20	8	1	0	1739455842145
12	12	29	-1	0	1739456411871
9	11	37	-1	0	1739457076833
12	21	31	-1	0	1739458098933
12	12	32	-1	0	1739458104689
12	4	3	-1	0	1739458680383
12	4	2	-1	0	1739458682816
12	30	2	-1	0	1739458684158
12	36	1	-1	0	1739459695157
12	37	1	-1	0	1739459703366
12	36	30	1	0	1739459705517
12	13	30	-1	0	1739459708219
12	43	8	1	0	1739459968276
12	41	7	1	0	1739459972873
12	16	3	-1	0	1739459973508
12	41	11	1	0	1739459977868
9	13	32	-1	0	1739460677580
9	13	34	-1	0	1739460684983
12	39	2	-1	0	1739461870871
12	49	0	-1	0	1739461885620
9	14	32	-1	0	1739462756572
9	14	33	-1	0	1739462764358
9	14	31	0	0	1739462768556
12	31	35	-1	0	1739463200655
12	2	35	-1	0	1739463202239
12	5	25	-1	0	1739463255107
12	56	35	-1	0	1739463966618
12	56	3	-1	0	1739463969892
12	56	23	-1	0	1739463986022
12	57	31	1	0	1739464351857
12	27	14	-1	0	1739465072407
12	35	18	-1	0	1739465074156
12	22	21	1	0	1739465081799
12	41	38	-1	0	1739465933279
12	22	38	-1	0	1739465943191
12	22	26	-1	0	1739465950589
12	59	27	-1	0	1739465967813
12	17	38	-1	0	1739466378699
12	17	39	-1	0	1739466383243
12	21	40	0	0	1739534278832
12	60	28	1	0	1739596338719
12	60	44	-1	0	1739596375576
12	60	45	-1	0	1739596514353
2	41	21	-1	0	1736699653945
2	76	59	-1	0	1736742910266
2	76	17	-1	0	1736742922963
2	76	61	-1	0	1736742927157
2	76	60	-1	0	1736742940395
2	76	62	0	0	1736742948462
2	76	41	0	0	1736742956561
5	0	0	0	0	1736789772253
5	0	13	-1	0	1736794790332
5	9	20	-1	0	1736797017857
5	14	19	1	0	1736803862843
5	14	7	1	0	1736803893027
5	14	4	0	0	1736803906487
5	14	14	-1	0	1736803922017
5	14	1	-1	0	1736803932872
5	18	10	1	0	1736814068084
2	78	62	0	0	1736827396977
2	78	34	0	0	1736827444195
2	78	40	-1	0	1736827474317
2	78	44	-1	0	1736827484951
2	78	28	0	0	1736827526564
2	11	67	-1	0	1736879445376
2	11	64	-1	0	1736879453796
2	11	65	-1	0	1736879462092
2	11	72	1	0	1736879469872
2	11	70	-1	0	1736879486997
2	11	63	-1	0	1736879490293
2	11	68	-1	0	1736879506976
2	82	30	1	0	1736880443638
2	82	28	-1	0	1736880826401
2	82	25	-1	0	1736880853621
2	85	55	-1	0	1736881894154
2	85	61	0	0	1736881898661
2	85	32	0	0	1736881913551
2	87	60	0	0	1736916170661
2	87	56	-1	0	1736917057003
2	88	59	-1	0	1736994658274
2	89	39	-1	0	1737002032595
2	90	2	1	0	1737041234908
2	90	6	1	0	1737041240352
2	90	39	1	0	1737041663568
2	90	23	0	0	1737041682232
2	91	46	-1	0	1737265267684
2	91	15	-1	0	1737265272225
2	91	58	-1	0	1737265275174
2	91	3	-1	0	1737265286526
2	91	47	0	0	1737265296108
2	91	33	-1	0	1737265307706
2	91	44	-1	0	1737265319366
2	91	48	1	0	1737265328679
3	7	5	-1	0	1737731277838
11	2	1	1	0	1739296934170
11	3	0	-1	0	1739296942915
11	5	1	1	0	1739296959223
11	6	1	0	0	1739296970122
11	11	4	-1	0	1739308727083
11	11	11	-1	0	1739351006932
12	0	5	-1	0	1739382053081
12	0	7	-1	0	1739382073497
12	0	8	-1	0	1739382082620
12	0	9	-1	0	1739382090311
12	2	4	-1	0	1739454349384
12	27	4	0	0	1739454897495
12	20	28	1	0	1739455523004
12	20	23	-1	0	1739455531052
12	21	28	1	0	1739455885661
12	12	28	1	0	1739456420384
12	12	11	1	0	1739456424980
12	12	8	1	0	1739456430078
12	20	6	-1	0	1739456441851
12	32	29	1	0	1739456444985
12	20	30	-1	0	1739456455882
12	18	27	1	0	1739457513110
12	2	7	-1	0	1739457523513
12	18	7	-1	0	1739457532375
12	12	31	-1	0	1739458099143
12	30	33	-1	0	1739458720958
12	30	32	-1	0	1739458737198
12	34	34	0	0	1739458836891
12	36	29	1	0	1739459718300
12	35	32	0	0	1739459720410
12	36	4	0	0	1739459724379
12	13	32	-1	0	1739459729156
12	36	19	-1	0	1739459752417
12	13	33	0	0	1739459754642
12	13	28	1	0	1739459772236
12	43	32	1	0	1739459972627
12	41	8	1	0	1739459999324
12	45	34	-1	0	1739460701318
12	49	4	-1	0	1739461881540
12	49	34	0	0	1739461889297
12	49	3	-1	0	1739461892336
12	49	27	-1	0	1739461898778
12	50	0	1	0	1739461906770
12	49	17	-1	0	1739461924889
12	49	2	-1	0	1739461942776
9	14	36	-1	0	1739462759041
9	14	29	0	0	1739462774076
12	5	14	-1	0	1739463242240
12	5	35	-1	0	1739463249614
12	56	30	0	0	1739464033007
12	11	20	-1	0	1739464378028
12	11	36	1	0	1739464385208
12	34	36	-1	0	1739464429345
12	27	35	0	0	1739465078919
12	27	21	1	0	1739465084062
12	20	18	-1	0	1739465098733
12	15	18	-1	0	1739465945548
12	41	37	-1	0	1739465946812
12	15	21	-1	0	1739465951967
12	22	35	-1	0	1739465954699
12	41	21	1	0	1739465957258
12	59	6	-1	0	1739465962089
12	59	3	-1	0	1739465964792
12	59	38	-1	0	1739465966741
12	49	37	-1	0	1739465967082
12	22	37	-1	0	1739465972076
12	59	26	-1	0	1739465972275
12	59	2	-1	0	1739465974211
12	59	13	-1	0	1739466022671
12	10	26	-1	0	1739466463257
12	60	26	-1	0	1739595941215
12	60	46	-1	0	1739596748921
2	41	40	-1	0	1736699659296
2	41	27	-1	0	1736699669496
2	41	56	-1	0	1736699678777
2	41	54	-1	0	1736699701623
2	41	44	-1	0	1736699714220
2	41	42	-1	0	1736699719063
2	41	34	-1	0	1736699726467
2	42	48	-1	0	1736699755072
2	42	6	0	0	1736699758063
2	42	30	1	0	1736699776467
2	42	2	0	0	1736699782193
2	42	1	0	0	1736699783500
2	43	54	-1	0	1736699797667
2	12	55	0	0	1736699805884
2	43	6	1	0	1736699806015
2	15	46	-1	0	1736699813840
2	12	45	-1	0	1736699815764
2	15	44	-1	0	1736699816356
2	43	1	1	0	1736699818330
2	43	35	-1	0	1736699825314
2	15	56	-1	0	1736699826258
2	43	33	-1	0	1736699830494
2	15	55	-1	0	1736699833393
2	15	53	-1	0	1736699836054
2	43	39	-1	0	1736699837015
2	15	54	-1	0	1736699841289
2	43	2	1	0	1736699841545
2	43	52	-1	0	1736699846338
2	15	52	-1	0	1736699847514
2	43	30	-1	0	1736699854545
2	43	46	-1	0	1736699858950
2	43	56	0	0	1736699868351
2	43	42	-1	0	1736699873522
2	43	47	-1	0	1736699881374
2	43	50	-1	0	1736699886121
2	43	10	-1	0	1736699889605
2	15	45	0	0	1736699890865
2	43	53	-1	0	1736699892193
2	43	15	-1	0	1736699895783
2	15	47	0	0	1736699899070
2	43	34	-1	0	1736699901631
2	43	55	-1	0	1736699904653
2	43	12	-1	0	1736699908231
2	43	38	-1	0	1736699910522
2	15	51	1	0	1736699912402
2	43	43	-1	0	1736699913062
2	15	49	0	0	1736699916750
2	15	43	-1	0	1736699918377
2	43	31	-1	0	1736699919235
2	43	49	-1	0	1736699922417
2	15	50	-1	0	1736699928826
2	43	51	-1	0	1736699929064
2	43	40	-1	0	1736699932600
2	15	42	-1	0	1736699934426
2	43	11	-1	0	1736699935702
2	43	37	-1	0	1736699943571
2	15	48	1	0	1736699946732
2	43	21	-1	0	1736699947512
2	43	44	-1	0	1736699950017
2	43	13	-1	0	1736699953745
2	43	48	1	0	1736699968653
2	15	41	0	0	1736699969975
2	12	32	0	0	1736699970866
2	43	3	-1	0	1736699973349
2	43	28	-1	0	1736699978967
2	12	51	0	0	1736699980424
2	43	9	-1	0	1736699980971
2	44	51	0	0	1736699984898
2	12	42	-1	0	1736699987389
2	43	32	0	0	1736699988209
2	44	1	1	0	1736699988923
2	12	52	-1	0	1736699994435
2	43	25	-1	0	1736699995377
2	43	45	0	0	1736700003046
2	44	30	-1	0	1736700004300
2	43	20	-1	0	1736700006697
2	44	9	-1	0	1736700008035
2	43	23	-1	0	1736700010894
2	44	54	-1	0	1736700015845
2	12	54	-1	0	1736700016113
2	44	6	1	0	1736700019083
2	43	41	0	0	1736700020204
2	12	40	-1	0	1736700022274
2	43	5	-1	0	1736700024063
2	44	39	-1	0	1736700027823
2	43	27	-1	0	1736700028279
2	44	32	-1	0	1736700031997
2	43	22	-1	0	1736700034627
2	44	56	-1	0	1736700040713
2	43	36	-1	0	1736700041563
2	44	2	1	0	1736700044045
2	43	24	-1	0	1736700046294
2	43	17	-1	0	1736700048482
2	44	38	-1	0	1736700052033
2	43	29	-1	0	1736700054948
2	44	49	-1	0	1736700056183
2	44	21	-1	0	1736700060141
2	12	39	0	0	1736700090805
2	12	35	-1	0	1736700098204
2	44	55	-1	0	1736700101758
2	12	46	-1	0	1736700102306
2	44	5	-1	0	1736700105476
2	44	13	-1	0	1736700112276
2	12	33	-1	0	1736700114498
2	12	38	-1	0	1736700118611
2	44	48	0	0	1736700120815
2	12	56	0	0	1736700131454
2	12	44	-1	0	1736700138801
2	44	53	0	0	1736700138846
2	44	29	-1	0	1736700150956
2	44	12	-1	0	1736700154245
2	12	34	-1	0	1736700155227
2	44	42	-1	0	1736700158086
2	44	31	-1	0	1736700162916
2	44	52	-1	0	1736700165985
2	44	40	-1	0	1736700169705
2	44	28	-1	0	1736700175588
2	44	41	0	0	1736700201919
2	44	44	0	0	1736700206167
2	44	10	0	0	1736700206466
2	45	33	-1	0	1736700557289
2	45	30	-1	0	1736700569636
2	45	9	-1	0	1736700574016
2	45	43	-1	0	1736700702931
2	45	1	1	0	1736700706495
2	45	2	1	0	1736700712655
2	45	6	1	0	1736700715515
2	45	55	-1	0	1736700730835
2	45	46	-1	0	1736700740896
2	45	56	-1	0	1736700755957
2	45	42	-1	0	1736700761236
2	45	34	-1	0	1736700776616
2	45	15	-1	0	1736700779800
2	45	17	-1	0	1736700782876
2	45	38	-1	0	1736700786037
2	45	40	-1	0	1736700789996
2	45	36	0	0	1736700799191
2	45	39	-1	0	1736700812035
2	45	3	-1	0	1736700816998
2	45	13	-1	0	1736700820216
2	45	50	-1	0	1736700825496
2	45	54	-1	0	1736700831995
2	45	37	-1	0	1736700854342
2	45	44	0	0	1736700859657
2	46	2	-1	0	1736700869757
2	45	21	-1	0	1736700871476
2	45	52	0	0	1736700875557
2	45	22	0	0	1736700887535
2	45	45	0	0	1736700902856
2	45	25	-1	0	1736700908095
2	45	53	-1	0	1736700912145
2	45	47	-1	0	1736700938420
2	45	31	-1	0	1736700943781
2	45	51	0	0	1736700966097
2	45	41	0	0	1736700984236
2	45	27	0	0	1736700990755
2	45	35	-1	0	1736700994516
2	45	23	-1	0	1736701002641
2	45	48	0	0	1736701021815
2	76	24	0	0	1736742971764
5	0	1	0	0	1736789783699
5	0	15	-1	0	1736794792291
5	10	9	0	0	1736797077204
5	10	5	-1	0	1736797087361
5	10	8	-1	0	1736797091848
5	10	17	-1	0	1736797102413
5	10	14	-1	0	1736797124857
5	10	7	-1	0	1736797133604
5	10	12	0	0	1736797183918
5	10	11	0	0	1736797186806
5	14	8	-1	0	1736803887981
5	18	12	1	0	1736814069886
5	18	3	-1	0	1736814074549
2	78	73	-1	0	1736827403724
2	78	36	-1	0	1736827417493
2	11	69	-1	0	1736879457762
2	11	61	1	0	1736879466283
2	11	71	0	0	1736879477764
2	11	66	-1	0	1736879503041
2	82	1	0	0	1736880459009
2	82	36	-1	0	1736880830578
2	85	24	-1	0	1736881916868
2	85	50	-1	0	1736881928820
2	87	34	0	0	1736916237343
2	87	43	-1	0	1736916243832
2	87	31	-1	0	1736916251641
2	87	28	1	0	1736916267008
2	87	37	-1	0	1736917071308
2	88	39	0	0	1736994681538
2	89	2	-1	0	1737002036899
2	90	13	-1	0	1737041238541
2	90	64	-1	0	1737041252337
2	90	35	-1	0	1737041256107
2	90	17	-1	0	1737041267616
2	90	43	1	0	1737041667957
2	90	36	1	0	1737041675124
2	91	13	-1	0	1737265269687
3	7	1	1	0	1737731278888
3	7	6	1	0	1737731284723
11	7	1	0	0	1739296989813
11	11	5	-1	0	1739308729645
11	11	12	-1	0	1739351728053
12	0	6	-1	0	1739382061115
9	10	29	0	0	1739454374672
12	27	0	-1	0	1739454902366
12	20	19	-1	0	1739455540024
12	21	29	0	0	1739455894307
12	21	19	-1	0	1739455902436
12	21	6	0	0	1739455925705
12	12	6	1	0	1739456434054
12	32	17	-1	0	1739456456231
12	22	29	0	0	1739456460900
12	32	23	-1	0	1739456464849
12	18	23	-1	0	1739457536854
12	4	7	0	0	1739457545651
12	22	32	-1	0	1739458149091
12	4	34	0	0	1739458790137
12	36	34	1	0	1739459728577
12	13	2	-1	0	1739459732214
12	40	3	-1	0	1739459974096
12	14	33	0	0	1739459976579
12	41	0	-1	0	1739459983890
12	16	28	1	0	1739460011877
12	27	23	-1	0	1739460036744
12	44	34	0	0	1739460054262
12	27	2	-1	0	1739460055863
9	13	31	-1	0	1739460714548
9	13	38	-1	0	1739460719117
9	13	37	-1	0	1739460724685
12	45	31	-1	0	1739460728486
12	50	27	-1	0	1739461921889
9	14	35	-1	0	1739462771948
9	14	38	-1	0	1739462775383
9	14	30	0	0	1739462776620
12	16	35	-1	0	1739463342801
12	18	14	-1	0	1739463342932
12	18	35	-1	0	1739463771284
12	56	0	1	0	1739464040929
12	56	6	-1	0	1739464045264
12	34	20	-1	0	1739464422796
12	3	37	-1	0	1739465097267
12	27	36	-1	0	1739465099268
12	12	18	-1	0	1739465101220
12	49	26	-1	0	1739465949398
12	49	38	-1	0	1739465951868
12	49	18	-1	0	1739465964062
12	22	36	-1	0	1739465964149
12	10	38	-1	0	1739466471728
12	60	41	0	0	1739595960905
2	45	11	-1	0	1736701007595
2	45	20	-1	0	1736701030176
2	45	49	-1	0	1736701066376
2	45	5	-1	0	1736701074036
2	45	12	-1	0	1736701079576
2	45	32	-1	0	1736701100280
2	45	28	-1	0	1736701107517
2	45	29	-1	0	1736701117097
2	45	24	0	0	1736701123356
2	45	10	-1	0	1736701128700
2	47	3	-1	0	1736701402309
2	47	2	-1	0	1736701417158
2	47	1	1	0	1736701420866
2	47	39	-1	0	1736701430182
2	47	33	-1	0	1736701434276
2	47	6	1	0	1736701437158
2	47	40	-1	0	1736701443759
2	47	30	-1	0	1736701448528
2	47	52	-1	0	1736701459946
2	47	43	-1	0	1736701462379
2	48	2	0	0	1736701484339
2	48	40	-1	0	1736701494586
2	48	6	1	0	1736701497318
2	49	44	-1	0	1736701502207
2	49	54	-1	0	1736701507965
2	49	30	0	0	1736701512435
2	49	6	-1	0	1736701515099
2	47	38	-1	0	1736701516439
2	49	38	-1	0	1736701517569
2	49	13	-1	0	1736701520569
2	47	55	-1	0	1736701523010
2	49	2	-1	0	1736701524307
2	49	25	-1	0	1736701527211
2	47	54	-1	0	1736701527759
2	49	33	-1	0	1736701531586
2	47	28	0	0	1736701535490
2	49	40	-1	0	1736701536799
2	49	36	-1	0	1736701541492
2	47	46	-1	0	1736701542277
2	49	56	1	0	1736701546867
2	49	12	-1	0	1736701551384
2	47	34	-1	0	1736701552259
2	47	49	-1	0	1736701563048
2	49	41	0	0	1736701571692
2	49	1	1	0	1736701574387
2	47	56	1	0	1736701577293
2	49	55	-1	0	1736701577884
2	49	20	0	0	1736701583105
2	49	50	0	0	1736701588429
2	49	9	-1	0	1736701592254
2	49	34	-1	0	1736701598667
2	47	37	-1	0	1736701602866
2	49	45	-1	0	1736701603167
2	49	46	-1	0	1736701607636
2	47	24	1	0	1736701611379
2	49	17	-1	0	1736701611988
2	47	13	-1	0	1736701614880
2	49	49	0	0	1736701622875
2	49	35	-1	0	1736701628256
2	49	10	-1	0	1736701631001
2	47	51	0	0	1736701632270
2	49	52	1	0	1736701635087
2	49	31	-1	0	1736701639196
2	47	47	-1	0	1736701641420
2	49	42	-1	0	1736701644026
2	47	21	-1	0	1736701647989
2	49	39	-1	0	1736701653872
2	47	15	-1	0	1736701654508
2	49	27	0	0	1736701662047
2	47	32	-1	0	1736701665796
2	49	32	-1	0	1736701669480
2	47	53	-1	0	1736701669745
2	50	2	0	0	1736701670364
2	49	5	0	0	1736701674150
2	47	35	-1	0	1736701675651
2	49	43	-1	0	1736701675952
2	50	30	-1	0	1736701679073
2	49	53	-1	0	1736701679449
2	50	6	1	0	1736701682285
2	49	48	-1	0	1736701684238
2	50	1	1	0	1736701684665
2	47	50	0	0	1736701687161
2	50	53	-1	0	1736701690190
2	47	42	-1	0	1736701694482
2	49	3	0	0	1736701697638
2	50	20	-1	0	1736701697673
2	50	46	-1	0	1736701700571
2	47	45	-1	0	1736701701234
2	47	12	-1	0	1736701706832
2	47	31	-1	0	1736701713383
2	47	27	1	0	1736701718301
2	49	24	-1	0	1736701719979
2	49	51	-1	0	1736701723500
2	47	44	-1	0	1736701726151
2	49	11	-1	0	1736701727540
2	47	9	-1	0	1736701728517
2	47	10	-1	0	1736701731522
2	49	29	0	0	1736701735938
2	47	20	-1	0	1736701741253
2	49	23	0	0	1736701741267
2	49	28	-1	0	1736701746354
2	49	37	-1	0	1736701751975
2	49	21	-1	0	1736701756033
2	49	47	-1	0	1736701761895
2	47	29	0	0	1736701762320
2	49	22	-1	0	1736701769976
2	49	15	-1	0	1736701772921
2	47	41	1	0	1736701783007
2	47	25	-1	0	1736701786629
2	51	30	0	0	1736701789123
2	51	2	1	0	1736701791585
2	51	46	-1	0	1736701799354
2	51	1	1	0	1736701800999
2	51	38	-1	0	1736701803882
2	51	49	0	0	1736701813269
2	51	6	1	0	1736701815293
2	47	57	-1	0	1736701815991
2	51	5	-1	0	1736701817204
2	51	10	-1	0	1736701820250
2	51	56	1	0	1736701834476
2	51	28	-1	0	1736701838531
2	51	42	-1	0	1736701843980
2	51	44	-1	0	1736701846754
2	51	33	-1	0	1736701859725
2	51	52	1	0	1736701863841
2	51	31	-1	0	1736701869285
2	51	57	0	0	1736701875022
2	47	58	-1	0	1736701878533
2	51	15	-1	0	1736701879287
2	47	22	0	0	1736701884332
2	51	39	-1	0	1736701884407
2	47	17	-1	0	1736701886847
2	51	43	-1	0	1736701887556
2	47	48	1	0	1736701891680
2	51	58	-1	0	1736701892673
2	51	9	-1	0	1736701895479
2	47	5	-1	0	1736701896453
2	51	40	-1	0	1736701900699
2	47	36	-1	0	1736701903113
2	47	23	-1	0	1736701910251
2	51	54	0	0	1736701918005
2	47	11	0	0	1736701918667
2	51	47	0	0	1736701938202
2	51	53	-1	0	1736701941501
2	51	13	-1	0	1736701944335
2	51	25	-1	0	1736701947622
2	51	12	-1	0	1736701955855
2	51	45	-1	0	1736701963658
2	51	34	1	0	1736701972373
2	51	20	0	0	1736701982702
2	51	41	1	0	1736701989767
2	51	32	-1	0	1736701995460
2	51	50	0	0	1736702001667
2	51	55	-1	0	1736702008585
2	52	2	0	0	1736702009868
2	51	35	-1	0	1736702014964
2	51	21	-1	0	1736702021408
2	51	36	-1	0	1736702026123
2	76	50	0	0	1736742974361
2	76	73	-1	0	1736743015745
5	0	2	0	0	1736789794334
5	0	14	-1	0	1736794796024
5	10	18	-1	0	1736797084878
5	10	13	-1	0	1736797097611
5	10	4	-1	0	1736797107644
5	10	6	-1	0	1736797162275
5	10	20	-1	0	1736797175269
5	10	10	0	0	1736797180395
5	10	3	-1	0	1736797182390
5	14	3	1	0	1736803929285
5	14	10	1	0	1736803939617
5	14	12	1	0	1736803948268
5	18	9	-1	0	1736814080859
5	18	6	-1	0	1736814093784
2	78	17	-1	0	1736827426386
2	78	67	0	0	1736827433076
2	78	30	1	0	1736827448279
2	11	58	1	0	1736879515632
2	82	40	-1	0	1736880464994
2	82	31	-1	0	1736880472551
2	82	54	0	0	1736880837845
2	86	30	-1	0	1736882248310
2	87	15	-1	0	1736916241789
2	87	47	-1	0	1736917084331
2	88	34	-1	0	1736994695369
2	89	1	0	0	1737002045401
2	90	37	-1	0	1737041264741
2	90	69	-1	0	1737041272436
2	90	38	-1	0	1737041278224
2	90	67	1	0	1737041698572
2	91	42	-1	0	1737265370945
2	91	72	0	0	1737265377818
2	91	59	-1	0	1737265386505
2	91	5	-1	0	1737265402132
2	91	40	-1	0	1737265410564
2	91	69	-1	0	1737265446490
2	91	31	-1	0	1737265467243
2	91	39	-1	0	1737265490409
2	91	17	-1	0	1737265511551
2	91	41	1	0	1737265524669
3	7	3	-1	0	1737731280259
11	8	1	0	0	1739296989991
11	11	6	-1	0	1739308763331
11	11	13	-1	0	1739351744800
12	0	11	-1	0	1739382106953
9	10	30	0	0	1739454378225
12	3	19	-1	0	1739454982281
12	11	19	1	0	1739455555952
12	21	27	1	0	1739455907860
12	12	19	-1	0	1739456444347
12	12	30	-1	0	1739456458872
12	11	7	1	0	1739457546314
12	3	32	0	0	1739458151256
12	2	34	0	0	1739458796734
12	36	2	-1	0	1739459731779
12	36	32	0	0	1739459737090
12	35	31	-1	0	1739459739355
12	36	6	-1	0	1739459741630
12	36	0	1	0	1739459747384
12	35	19	0	0	1739459748592
12	13	34	-1	0	1739459748672
12	41	28	1	0	1739459993649
12	16	32	0	0	1739459995575
12	16	11	0	0	1739460005382
9	13	35	-1	0	1739460721333
12	49	23	-1	0	1739461933825
12	34	14	0	0	1739462812254
12	18	25	-1	0	1739463334962
12	0	36	0	0	1739464053287
12	56	14	-1	0	1739464067852
12	16	36	-1	0	1739464689697
12	30	37	-1	0	1739465121543
12	15	37	-1	0	1739465956037
12	16	38	-1	0	1739465961933
12	16	26	-1	0	1739465969826
12	59	14	-1	0	1739465975550
12	10	21	-1	0	1739466503036
12	60	3	-1	0	1739595964009
2	51	48	1	0	1736702031388
2	12	72	-1	0	1736743875754
5	0	3	0	0	1736789804780
5	5	1	1	0	1736794871154
5	5	6	0	0	1736794905169
5	10	1	0	0	1736797104164
5	10	16	0	0	1736797126414
5	10	15	-1	0	1736797172074
5	10	19	-1	0	1736797179073
5	10	2	0	0	1736797185600
5	11	13	-1	0	1736797829001
5	11	20	-1	0	1736797851410
5	11	4	-1	0	1736797867036
5	11	10	1	0	1736797878147
5	11	17	-1	0	1736797896464
5	11	12	-1	0	1736797900727
5	14	11	1	0	1736803951271
5	18	21	1	0	1736814082623
5	18	11	1	0	1736814100229
5	18	14	1	0	1736814119704
2	78	58	0	0	1736827463936
2	78	45	-1	0	1736827499162
2	78	54	-1	0	1736827518134
2	78	59	0	0	1736827533095
2	78	11	0	0	1736827555332
2	11	62	0	0	1736879541867
2	83	2	0	0	1736880472180
2	83	1	0	0	1736880476447
2	82	69	1	0	1736880481752
2	82	70	1	0	1736880510791
2	82	9	-1	0	1736880858289
2	82	5	-1	0	1736880861329
2	86	1	-1	0	1736882274448
2	86	34	-1	0	1736882288352
2	87	5	-1	0	1736916248606
2	87	32	0	0	1736917156922
2	88	27	-1	0	1736994703869
2	89	36	0	0	1737002065790
2	90	72	0	0	1737041306019
2	90	42	0	0	1737041712900
2	90	15	-1	0	1737041716312
2	91	67	-1	0	1737265374894
2	91	53	-1	0	1737265380822
2	91	24	1	0	1737265398212
2	91	27	-1	0	1737265404935
2	91	37	0	0	1737265419685
2	91	12	-1	0	1737265429110
2	91	66	-1	0	1737265441153
2	91	10	-1	0	1737265448799
2	91	29	-1	0	1737265459772
2	91	62	0	0	1737265477875
11	8	0	0	0	1739296990912
11	11	7	-1	0	1739316118551
11	14	10	-1	0	1739362870292
12	1	8	0	0	1739382153182
12	3	4	-1	0	1739454392223
12	3	0	1	0	1739454402014
12	10	19	-1	0	1739454990485
12	28	17	-1	0	1739455006171
12	28	4	-1	0	1739455018446
12	28	23	-1	0	1739455035130
12	11	28	1	0	1739455563063
12	21	23	-1	0	1739455914322
12	21	8	1	0	1739455921439
12	5	30	-1	0	1739456469600
12	18	30	1	0	1739457550869
12	18	11	0	0	1739457558832
12	9	32	1	0	1739458151281
12	3	7	0	0	1739458163328
12	5	34	-1	0	1739458889274
12	13	1	-1	0	1739459737919
12	36	17	-1	0	1739459783798
12	14	11	1	0	1739459995272
12	40	27	1	0	1739460001662
12	17	8	0	0	1739460004816
12	40	8	1	0	1739460008789
12	27	13	-1	0	1739460030931
9	13	30	0	0	1739460728817
12	49	32	0	0	1739461946507
12	52	3	-1	0	1739462818977
12	53	14	-1	0	1739462850016
12	53	0	1	0	1739462858503
12	52	4	-1	0	1739462863371
12	52	33	-1	0	1739462870650
12	11	33	-1	0	1739463347735
12	3	35	-1	0	1739463405801
12	56	36	0	0	1739464074653
12	56	31	-1	0	1739464080503
12	16	20	-1	0	1739464700914
12	53	18	-1	0	1739465154049
12	53	32	1	0	1739465236103
12	59	35	-1	0	1739465989371
12	59	17	-1	0	1739465993781
12	59	23	-1	0	1739466000635
12	11	39	-1	0	1739466513845
12	11	38	-1	0	1739466517605
12	10	39	-1	0	1739466535203
12	10	37	-1	0	1739466537759
12	10	18	-1	0	1739466548849
12	60	0	1	0	1739595979362
12	60	18	0	0	1739596000790
2	51	29	0	0	1736702049081
2	51	17	-1	0	1736702052757
2	51	22	0	0	1736702062772
2	51	3	-1	0	1736702066792
2	51	37	1	0	1736702074209
2	51	27	0	0	1736702080837
2	51	51	1	0	1736702091336
2	51	23	-1	0	1736702097771
2	51	24	1	0	1736702114274
2	51	11	-1	0	1736702118277
2	13	57	-1	0	1736702201855
2	13	58	1	0	1736702205859
2	53	30	-1	0	1736702311393
2	52	45	-1	0	1736702359934
2	52	46	0	0	1736702369786
2	52	6	0	0	1736702375073
2	52	30	1	0	1736702379935
2	52	52	0	0	1736702383952
2	52	1	0	0	1736702386713
2	52	38	1	0	1736702396212
2	52	25	-1	0	1736702404652
2	52	11	1	0	1736702421921
2	53	57	-1	0	1736702428725
2	51	59	-1	0	1736702429778
2	53	43	-1	0	1736702432595
2	53	6	-1	0	1736702434874
2	52	56	0	0	1736702435144
2	52	20	-1	0	1736702440583
2	53	58	-1	0	1736702443342
2	52	31	0	0	1736702451363
2	52	58	0	0	1736702455973
2	53	2	-1	0	1736702459158
2	52	59	0	0	1736702462694
2	53	1	1	0	1736702462987
2	52	55	-1	0	1736702468105
2	52	27	-1	0	1736702480145
2	53	56	-1	0	1736702484975
2	52	57	0	0	1736702485573
2	52	3	-1	0	1736702491813
2	52	15	-1	0	1736702496544
2	53	5	-1	0	1736702500422
2	52	47	-1	0	1736702502193
2	53	40	-1	0	1736702505238
2	52	54	-1	0	1736702510061
2	53	39	-1	0	1736702514381
2	52	53	1	0	1736702522038
2	53	51	-1	0	1736702530863
2	52	35	1	0	1736702533213
2	53	12	-1	0	1736702535363
2	52	12	-1	0	1736702538163
2	53	45	-1	0	1736702539965
2	52	33	1	0	1736702548437
2	53	37	-1	0	1736702554324
2	52	29	-1	0	1736702554547
2	52	28	0	0	1736702558813
2	53	25	-1	0	1736702559646
2	53	50	-1	0	1736702568055
2	52	13	0	0	1736702571944
2	53	55	-1	0	1736702572140
2	52	43	0	0	1736702576283
2	52	48	-1	0	1736702582234
2	53	22	1	0	1736702586576
2	53	47	-1	0	1736702597443
2	53	31	-1	0	1736702602178
2	53	59	1	0	1736702612548
2	53	13	-1	0	1736702615268
2	53	44	-1	0	1736702617691
2	53	53	-1	0	1736702621400
2	53	54	-1	0	1736702626049
2	53	49	-1	0	1736702631072
2	53	17	-1	0	1736702634348
2	53	42	-1	0	1736702641107
2	53	52	1	0	1736702649866
2	53	38	-1	0	1736702657006
2	53	46	-1	0	1736702660545
2	53	20	-1	0	1736702664141
2	53	33	-1	0	1736702669077
2	53	35	-1	0	1736702673952
2	53	36	-1	0	1736702681271
2	53	34	-1	0	1736702688130
2	53	28	-1	0	1736702696043
2	53	21	-1	0	1736702702662
2	53	27	-1	0	1736702711767
2	53	48	-1	0	1736702717234
2	53	15	-1	0	1736702720911
2	53	29	-1	0	1736702727212
2	53	23	-1	0	1736702732325
2	52	60	-1	0	1736702737534
2	53	11	-1	0	1736702737870
2	53	10	-1	0	1736702741965
2	53	9	-1	0	1736702743706
2	52	40	0	0	1736702748161
2	53	60	-1	0	1736702748412
2	53	3	-1	0	1736702753279
2	52	39	-1	0	1736702758325
2	53	41	1	0	1736702775893
2	53	24	1	0	1736702785519
2	53	32	-1	0	1736702799857
2	13	59	0	0	1736702966144
2	52	42	-1	0	1736703041771
2	52	10	-1	0	1736703049199
2	52	36	-1	0	1736703053892
2	52	51	0	0	1736703089082
2	52	17	-1	0	1736703096132
2	52	5	-1	0	1736703100081
2	52	41	-1	0	1736703129662
2	52	32	-1	0	1736703146749
2	52	44	0	0	1736703179863
2	52	22	1	0	1736703187114
2	54	2	-1	0	1736703188193
2	52	37	-1	0	1736703191640
2	54	1	1	0	1736703198738
2	54	30	-1	0	1736703202234
2	54	6	1	0	1736703204322
2	54	38	0	0	1736703207132
2	52	34	1	0	1736703210040
2	54	3	-1	0	1736703211430
2	52	23	-1	0	1736703214832
2	52	9	0	0	1736703223482
2	52	21	-1	0	1736703236524
2	52	50	0	0	1736703242831
2	52	49	0	0	1736703245602
2	52	24	0	0	1736703255883
2	55	2	0	0	1736703595015
2	55	3	-1	0	1736703598707
2	55	1	1	0	1736703601285
2	55	38	-1	0	1736703607230
2	55	6	1	0	1736703608907
2	55	60	0	0	1736703621304
2	55	46	-1	0	1736703623921
2	55	44	-1	0	1736703627256
2	55	30	-1	0	1736703633262
2	55	5	-1	0	1736703635995
2	55	58	-1	0	1736703646649
2	55	42	-1	0	1736703649571
2	55	33	-1	0	1736703653173
2	55	57	0	0	1736703661404
2	55	52	0	0	1736703665046
2	55	37	0	0	1736703684050
2	55	9	-1	0	1736703687808
2	55	13	-1	0	1736703692290
2	55	55	-1	0	1736703695877
2	55	17	-1	0	1736703698862
2	55	49	0	0	1736703706804
2	55	27	-1	0	1736703712492
2	1	12	0	0	1736703786875
2	52	61	-1	0	1736704545216
2	56	6	0	0	1736708201431
2	56	30	-1	0	1736708208219
2	56	2	0	0	1736708214394
2	56	43	-1	0	1736708216857
2	56	61	0	0	1736708220514
2	56	44	-1	0	1736708226261
2	56	1	0	0	1736708227976
2	56	58	-1	0	1736708240491
2	56	39	-1	0	1736708254476
2	56	56	1	0	1736708279233
2	56	38	-1	0	1736708283034
2	56	36	-1	0	1736708288446
2	56	46	-1	0	1736708292455
2	77	6	1	0	1736744241323
2	77	30	1	0	1736744245647
2	77	73	0	0	1736744269020
5	0	4	0	0	1736789850858
5	5	13	-1	0	1736794881251
5	5	10	0	0	1736794884823
5	11	18	-1	0	1736797833114
5	11	19	-1	0	1736797847041
5	11	14	-1	0	1736797859337
5	11	15	-1	0	1736797875975
5	11	7	-1	0	1736797881628
5	11	9	1	0	1736797902387
5	11	2	1	0	1736797905966
5	15	1	1	0	1736804657932
5	15	10	1	0	1736804662893
5	15	18	0	0	1736804704337
5	15	17	-1	0	1736804710102
5	18	2	-1	0	1736814095796
5	18	16	1	0	1736814098743
5	18	1	1	0	1736814102209
5	18	22	-1	0	1736814123257
2	78	53	-1	0	1736827468137
2	78	9	-1	0	1736827480759
2	11	57	0	0	1736879545624
2	83	6	0	0	1736880473928
2	82	65	0	0	1736880500110
2	82	37	-1	0	1736880865771
2	86	2	0	0	1736882282437
2	86	6	1	0	1736882294879
2	86	10	-1	0	1736882299263
2	87	30	-1	0	1736916279479
2	87	36	-1	0	1736917161405
2	88	42	-1	0	1736994713392
2	88	31	-1	0	1736994732802
2	89	71	0	0	1737002076928
2	89	6	0	0	1737002079542
2	90	21	-1	0	1737041311263
2	90	59	-1	0	1737041721872
2	91	38	-1	0	1737265382774
11	12	5	-1	0	1739316195978
11	15	11	-1	0	1739363141590
12	0	12	-1	0	1739382186106
12	4	4	-1	0	1739454485169
12	5	0	-1	0	1739454500394
12	28	19	-1	0	1739454999035
12	10	23	-1	0	1739455036634
12	11	23	1	0	1739455569207
12	2	28	1	0	1739455574898
12	29	28	1	0	1739455585276
12	11	6	1	0	1739455959399
12	22	11	-1	0	1739456506995
12	18	6	-1	0	1739457562563
12	11	32	1	0	1739458152234
12	9	31	-1	0	1739458159061
12	22	7	0	0	1739458159201
12	34	4	-1	0	1739458336115
12	9	2	-1	0	1739459070855
12	9	33	-1	0	1739459079222
12	20	1	-1	0	1739459748266
12	20	34	-1	0	1739459754886
12	36	23	-1	0	1739459758601
12	20	13	-1	0	1739459763458
12	13	23	1	0	1739459781040
12	22	2	-1	0	1739459801120
12	13	11	1	0	1739459802894
12	40	1	1	0	1739459805372
12	7	13	-1	0	1739459808047
12	38	2	0	0	1739459809059
12	22	3	-1	0	1739459815222
12	38	23	-1	0	1739459834727
12	7	19	0	0	1739459838347
12	7	6	-1	0	1739459878480
12	16	23	-1	0	1739459892341
12	16	2	0	0	1739459900106
12	7	32	1	0	1739459900692
12	41	2	-1	0	1739459903397
12	7	28	1	0	1739459905392
12	43	23	-1	0	1739459909587
12	38	6	-1	0	1739459910118
12	14	30	-1	0	1739459912783
12	27	1	-1	0	1739460040966
12	45	7	-1	0	1739460817172
12	45	29	0	0	1739460854224
12	49	33	-1	0	1739461950091
12	49	1	-1	0	1739461953946
12	34	25	-1	0	1739462821748
12	55	1	-1	0	1739463386580
12	17	25	-1	0	1739463395815
12	55	35	-1	0	1739463398332
12	3	25	-1	0	1739463408579
12	39	25	-1	0	1739464091042
12	15	20	-1	0	1739464111902
12	53	6	-1	0	1739464115935
12	39	14	-1	0	1739464131191
12	21	14	-1	0	1739464135044
12	12	36	-1	0	1739464137133
12	29	22	-1	0	1739464137244
12	29	6	-1	0	1739464143494
12	35	17	-1	0	1739464148418
12	20	22	-1	0	1739464148605
12	30	22	-1	0	1739464152623
12	30	20	-1	0	1739464160984
12	16	22	-1	0	1739464708793
12	30	21	-1	0	1739465156186
12	0	39	0	0	1739466009680
12	59	37	-1	0	1739466034854
12	11	26	-1	0	1739466522747
12	60	6	-1	0	1739595984877
12	60	1	-1	0	1739595993792
12	60	40	-1	0	1739596003122
2	56	48	1	0	1736708310459
2	56	59	-1	0	1736708317774
2	56	42	-1	0	1736708324936
2	56	31	-1	0	1736708332341
2	56	47	-1	0	1736708336376
2	56	40	-1	0	1736708340994
2	56	12	-1	0	1736708344766
2	56	53	-1	0	1736708349076
2	56	34	-1	0	1736708358896
2	56	33	-1	0	1736708367726
2	56	35	-1	0	1736708378408
2	56	51	1	0	1736708387347
2	56	60	-1	0	1736708403085
2	56	22	-1	0	1736708406126
2	56	21	-1	0	1736708414850
2	56	55	-1	0	1736708418809
2	56	57	0	0	1736708424226
2	56	20	0	0	1736708433797
2	56	9	-1	0	1736708439656
2	56	17	-1	0	1736708442516
2	56	27	-1	0	1736708453844
2	56	5	-1	0	1736708461201
2	56	54	-1	0	1736708471361
2	56	28	0	0	1736708482940
2	56	29	-1	0	1736708488801
2	56	49	0	0	1736708495425
2	56	41	0	0	1736708518381
2	56	10	-1	0	1736708520901
2	56	15	-1	0	1736708524180
2	56	32	-1	0	1736708537821
2	56	13	-1	0	1736708543071
2	56	11	0	0	1736708553126
2	56	23	-1	0	1736708571480
2	56	25	-1	0	1736708574799
2	56	3	-1	0	1736708581322
2	56	50	0	0	1736708585379
2	56	52	-1	0	1736708589281
2	56	37	1	0	1736708602561
2	56	45	-1	0	1736708620860
2	56	24	-1	0	1736708627640
2	57	59	-1	0	1736708752177
2	57	1	1	0	1736708777568
2	57	60	-1	0	1736708814067
2	57	6	1	0	1736708817227
2	57	56	0	0	1736708831872
2	57	23	0	0	1736708841387
2	57	2	0	0	1736708847125
2	57	30	1	0	1736708852389
2	57	58	0	0	1736708867027
2	57	25	-1	0	1736708870309
2	57	53	-1	0	1736708873028
2	57	61	-1	0	1736708877062
2	57	40	-1	0	1736708880329
2	56	62	-1	0	1736708882064
2	57	12	-1	0	1736708883506
2	57	9	-1	0	1736708885268
2	57	5	0	0	1736708889867
2	57	39	-1	0	1736708898468
2	57	35	-1	0	1736708903987
2	57	54	-1	0	1736708907287
2	57	62	0	0	1736708913407
2	57	42	-1	0	1736708917509
2	57	57	0	0	1736708921407
2	57	36	0	0	1736708927527
2	57	22	0	0	1736708933252
2	57	45	-1	0	1736708950727
2	57	31	-1	0	1736708955267
2	57	17	-1	0	1736708957870
2	57	48	-1	0	1736708968599
2	57	38	-1	0	1736708972308
2	57	37	-1	0	1736708981087
2	57	43	-1	0	1736708983150
2	57	13	-1	0	1736708987887
2	57	41	-1	0	1736709000707
2	57	10	-1	0	1736709004747
2	57	33	0	0	1736709017908
2	57	44	-1	0	1736709020257
2	57	34	-1	0	1736709047797
2	57	28	-1	0	1736709054427
2	57	46	-1	0	1736709058425
2	57	27	-1	0	1736709079767
2	57	47	-1	0	1736709085808
2	57	21	-1	0	1736709094307
2	57	20	0	0	1736709099888
2	57	24	-1	0	1736709106397
2	57	51	-1	0	1736709110287
2	57	11	-1	0	1736709116431
2	57	55	-1	0	1736709122178
2	57	50	0	0	1736709126844
2	57	49	1	0	1736709132187
2	57	15	-1	0	1736709137989
2	57	29	0	0	1736709149726
2	57	3	-1	0	1736709157106
2	0	63	0	0	1736709159375
2	57	32	0	0	1736709163330
2	57	63	-1	0	1736709168047
2	57	52	1	0	1736709173131
2	58	34	-1	0	1736717867290
2	58	30	1	0	1736717875649
2	58	2	0	0	1736717885335
2	58	56	1	0	1736717894509
2	58	61	1	0	1736717909325
2	58	1	1	0	1736717913753
2	58	6	1	0	1736717920371
2	58	63	-1	0	1736717923789
2	58	48	0	0	1736717931217
2	59	30	1	0	1736719950197
2	59	31	-1	0	1736719955844
2	59	17	-1	0	1736719958348
2	59	63	-1	0	1736719960995
2	59	61	-1	0	1736719964617
2	59	48	-1	0	1736719969589
2	59	2	0	0	1736719973342
2	59	1	0	0	1736719976394
2	59	38	0	0	1736719978703
2	59	60	-1	0	1736719983016
2	59	6	0	0	1736719985270
2	59	28	-1	0	1736719990036
2	59	64	-1	0	1736720059498
2	59	65	-1	0	1736720096363
2	59	59	0	0	1736720103016
2	59	58	1	0	1736720107985
2	59	33	1	0	1736720112516
2	59	42	-1	0	1736720118233
2	59	15	-1	0	1736720121065
2	59	9	-1	0	1736720122325
2	59	62	0	0	1736720126662
2	59	13	0	0	1736720230177
2	59	56	-1	0	1736720235066
2	59	25	0	0	1736720238532
2	59	53	-1	0	1736720241191
2	59	20	-1	0	1736720243181
2	59	51	-1	0	1736720245345
2	59	44	-1	0	1736720248598
2	59	55	-1	0	1736720251697
2	59	39	0	0	1736720256005
2	16	58	1	0	1736720771791
2	16	63	-1	0	1736720774935
2	16	35	-1	0	1736720783147
2	16	64	0	0	1736720788604
2	16	61	0	0	1736720791589
2	16	65	0	0	1736720795280
2	16	46	-1	0	1736720798129
2	16	60	-1	0	1736720802065
2	16	39	-1	0	1736720810971
2	16	40	-1	0	1736720814638
2	16	56	0	0	1736720824136
2	16	42	-1	0	1736720828014
2	16	34	-1	0	1736720834950
2	16	50	-1	0	1736720839249
2	16	47	-1	0	1736720844321
2	16	59	-1	0	1736720851495
2	16	57	0	0	1736720856111
2	16	44	-1	0	1736720858504
2	16	55	-1	0	1736720862139
2	16	51	0	0	1736720868078
2	16	38	-1	0	1736720869442
2	16	53	0	0	1736720877446
2	16	45	-1	0	1736720896430
2	77	1	1	0	1736744251396
5	0	5	0	0	1736789904628
5	5	9	0	0	1736794883233
5	5	11	0	0	1736794886167
5	11	5	-1	0	1736797835340
5	11	16	1	0	1736797861249
5	11	8	-1	0	1736797873337
5	11	6	-1	0	1736797894087
5	11	3	-1	0	1736797899025
5	11	1	-1	0	1736797903836
5	11	11	1	0	1736797908536
5	15	9	1	0	1736804661413
5	15	11	1	0	1736804674795
5	15	14	-1	0	1736804688920
5	15	8	-1	0	1736804697558
5	18	20	-1	0	1736814126799
2	78	5	-1	0	1736827505047
2	78	71	0	0	1736827510297
2	79	2	0	0	1736879781103
2	82	45	-1	0	1736880520612
2	82	61	-1	0	1736880875080
2	86	67	-1	0	1736882292612
2	86	13	-1	0	1736882297118
2	86	25	-1	0	1736882301196
2	86	73	-1	0	1736882308907
2	86	47	-1	0	1736882325652
2	86	48	1	0	1736882335305
2	87	55	-1	0	1736916292356
2	87	65	-1	0	1736917166775
2	88	38	-1	0	1736994739536
2	89	38	-1	0	1737002092085
2	89	3	-1	0	1737002107577
2	90	73	0	0	1737041340495
2	90	25	-1	0	1737041344798
2	90	70	-1	0	1737041349462
2	90	71	-1	0	1737041734341
2	91	21	-1	0	1737265389964
2	91	43	-1	0	1737265392375
2	91	25	-1	0	1737265407432
2	91	45	-1	0	1737265425852
11	12	8	-1	0	1739316197460
9	3	0	-1	0	1739371790539
12	0	13	-1	0	1739382202235
12	5	17	-1	0	1739454492656
12	5	4	-1	0	1739454496859
12	28	0	1	0	1739455030241
12	22	28	1	0	1739455574330
12	11	27	1	0	1739455577064
12	4	6	-1	0	1739456037207
12	0	30	0	0	1739456061556
12	2	6	-1	0	1739456065448
9	11	32	-1	0	1739456075239
12	32	30	1	0	1739456513592
12	18	29	0	0	1739457589370
12	10	6	-1	0	1739457603003
12	10	30	1	0	1739457621995
12	18	8	0	0	1739457633141
12	22	31	0	0	1739458153948
12	3	31	-1	0	1739458170221
12	9	3	-1	0	1739459074177
12	20	2	-1	0	1739459751268
12	44	23	-1	0	1739460044149
12	27	30	0	0	1739460051022
12	44	17	0	0	1739460070308
12	27	31	-1	0	1739460081901
12	27	28	1	0	1739460089903
12	27	11	1	0	1739460121409
12	44	13	-1	0	1739460137309
12	45	33	-1	0	1739460876211
12	49	11	1	0	1739461968778
12	49	28	0	0	1739461983286
12	49	8	1	0	1739461986884
12	52	0	-1	0	1739462822428
12	36	25	-1	0	1739462825143
12	52	17	-1	0	1739462825592
12	52	13	-1	0	1739462828809
12	52	27	-1	0	1739462834097
12	52	14	-1	0	1739462841000
12	52	25	-1	0	1739462855027
12	10	14	-1	0	1739462875295
12	17	22	-1	0	1739463402218
12	3	14	-1	0	1739463412827
12	17	35	-1	0	1739463416494
12	17	14	-1	0	1739463435159
12	11	14	1	0	1739463477036
12	11	35	-1	0	1739463495377
12	11	3	-1	0	1739463500913
12	11	25	-1	0	1739463506986
12	18	36	0	0	1739464094706
12	36	22	-1	0	1739464105238
12	41	20	-1	0	1739464108476
12	49	35	-1	0	1739464109704
12	36	20	1	0	1739464109716
12	41	35	-1	0	1739464111009
12	36	35	-1	0	1739464115819
12	15	22	0	0	1739464124557
12	56	11	0	0	1739464133083
12	35	0	1	0	1739464136802
12	53	35	-1	0	1739464141905
12	21	20	0	0	1739464142532
12	35	13	-1	0	1739464142582
12	10	35	-1	0	1739464155736
12	15	25	-1	0	1739464158351
12	39	34	1	0	1739464161360
12	15	13	-1	0	1739464163556
12	57	29	1	0	1739464168434
12	56	1	-1	0	1739464169183
12	39	17	-1	0	1739464181678
12	57	0	-1	0	1739464183891
12	39	8	-1	0	1739464188668
12	58	0	1	0	1739464199803
12	35	35	-1	0	1739464200136
12	10	22	-1	0	1739464200752
12	15	7	-1	0	1739464200867
12	20	25	-1	0	1739464205373
12	10	20	-1	0	1739464205420
12	53	3	-1	0	1739464213232
12	57	27	-1	0	1739464226194
12	35	11	0	0	1739464243572
12	15	34	-1	0	1739464245852
12	29	31	-1	0	1739464253874
12	40	20	1	0	1739464254875
12	56	4	-1	0	1739464280983
12	31	20	-1	0	1739464282668
12	57	22	-1	0	1739464287820
12	29	11	1	0	1739464295544
12	35	6	-1	0	1739464296574
12	57	13	1	0	1739464303072
12	29	32	1	0	1739464323533
12	0	37	0	0	1739464787552
12	53	31	-1	0	1739465172200
12	53	2	-1	0	1739465187004
12	2	39	-1	0	1739466053426
12	59	36	-1	0	1739466068218
12	16	39	-1	0	1739466080998
12	59	7	-1	0	1739466085988
12	21	39	-1	0	1739467342727
12	60	19	-1	0	1739595989018
2	16	33	1	0	1736720882348
2	16	43	-1	0	1736720898525
2	16	62	0	0	1736720912657
2	16	54	-1	0	1736720918312
2	16	49	0	0	1736720923876
2	16	37	-1	0	1736720933827
2	16	36	-1	0	1736720939974
2	16	48	1	0	1736720947347
2	16	52	-1	0	1736720950606
2	16	41	0	0	1736720962172
2	59	27	-1	0	1736721238044
2	59	54	-1	0	1736721246270
2	59	21	-1	0	1736721251019
2	59	35	0	0	1736721257689
2	59	43	0	0	1736721260429
2	59	5	-1	0	1736721263124
2	59	34	1	0	1736721268190
2	59	22	1	0	1736721272780
2	59	32	-1	0	1736721280000
2	59	40	0	0	1736721285660
2	59	45	-1	0	1736721288399
2	59	47	0	0	1736721292719
2	59	46	-1	0	1736721298796
2	59	52	-1	0	1736721306652
2	59	3	-1	0	1736721309622
2	59	12	0	0	1736721312615
2	59	37	-1	0	1736721315438
2	59	50	0	0	1736721318699
2	59	10	-1	0	1736721321109
2	59	29	-1	0	1736721326515
2	59	36	0	0	1736721331594
2	59	57	0	0	1736721334333
2	59	11	0	0	1736721339179
2	59	41	-1	0	1736721346822
2	59	23	-1	0	1736721349095
2	59	49	0	0	1736721351928
2	59	24	0	0	1736721356453
2	59	66	-1	0	1736721388336
2	59	67	-1	0	1736721430204
2	59	68	-1	0	1736722158717
2	16	66	-1	0	1736724723108
2	16	68	-1	0	1736724728450
2	16	67	0	0	1736724744898
2	34	30	0	0	1736727242451
2	34	2	0	0	1736727249792
2	34	1	0	0	1736727251987
2	60	2	-1	0	1736732431789
2	60	1	1	0	1736732434214
2	60	6	1	0	1736732435774
2	60	60	-1	0	1736732443817
2	60	30	-1	0	1736732448409
2	60	65	-1	0	1736732452818
2	60	42	-1	0	1736732466875
2	60	58	-1	0	1736732474541
2	60	22	-1	0	1736732478445
2	60	34	-1	0	1736732482560
2	60	21	-1	0	1736732487926
2	60	66	-1	0	1736732490774
2	60	68	-1	0	1736732498150
2	60	47	0	0	1736732513338
2	60	61	0	0	1736732517268
2	60	38	1	0	1736732521586
2	60	25	0	0	1736732528223
2	60	20	0	0	1736732534861
2	60	67	1	0	1736732546492
2	60	59	-1	0	1736732550703
2	60	33	-1	0	1736732565430
2	60	55	-1	0	1736732571730
2	60	12	0	0	1736732630665
2	60	62	-1	0	1736732639610
2	60	40	0	0	1736732650485
2	60	63	-1	0	1736732655432
2	60	37	0	0	1736732662287
2	60	52	-1	0	1736732671554
2	60	43	0	0	1736732681265
2	60	48	1	0	1736732694933
2	60	46	-1	0	1736732701600
2	60	56	0	0	1736732709251
2	60	64	-1	0	1736732714432
2	60	10	-1	0	1736732719910
2	60	51	0	0	1736732724986
2	60	53	-1	0	1736732728792
2	60	35	-1	0	1736732738293
2	60	23	1	0	1736732744494
2	60	13	-1	0	1736732750944
2	60	27	1	0	1736732758341
2	60	39	-1	0	1736732765309
2	60	44	0	0	1736732774663
2	60	15	-1	0	1736732777880
2	60	31	-1	0	1736732781160
2	60	3	0	0	1736732784633
2	60	36	-1	0	1736732792049
2	60	45	-1	0	1736732797252
2	60	57	1	0	1736732802085
2	60	54	-1	0	1736732807906
2	60	11	0	0	1736732815973
2	60	28	-1	0	1736732821644
2	60	32	-1	0	1736732831226
2	60	29	0	0	1736732838747
2	60	9	-1	0	1736732842649
2	60	5	1	0	1736732845214
2	60	17	-1	0	1736732847479
2	60	41	1	0	1736732859741
2	60	50	-1	0	1736732868299
2	60	49	-1	0	1736732875624
2	60	24	0	0	1736732883713
2	13	60	-1	0	1736734004732
2	13	66	-1	0	1736734008198
2	13	68	0	0	1736734014472
2	13	64	-1	0	1736734022253
2	13	65	-1	0	1736734031695
2	13	63	-1	0	1736734034046
2	13	62	-1	0	1736734035893
2	13	61	0	0	1736734040440
2	13	67	0	0	1736734051069
2	61	30	-1	0	1736734107433
2	61	2	0	0	1736734111976
2	61	6	1	0	1736734114354
2	61	38	-1	0	1736734116885
2	61	27	-1	0	1736734121499
2	61	60	0	0	1736734130554
2	61	1	1	0	1736734133218
2	61	66	-1	0	1736734136596
2	62	6	1	0	1736734137904
2	61	23	-1	0	1736734140031
2	61	64	-1	0	1736734143407
2	61	62	0	0	1736734156572
2	61	68	-1	0	1736734161648
2	61	9	-1	0	1736734163816
2	61	56	-1	0	1736734170952
2	61	61	1	0	1736734173102
2	61	58	0	0	1736734182246
2	61	5	0	0	1736734190785
2	61	59	-1	0	1736734194765
2	61	39	-1	0	1736734199469
2	61	63	-1	0	1736734203575
2	61	54	-1	0	1736734206660
2	61	10	-1	0	1736734209203
2	61	53	-1	0	1736734212239
2	61	40	-1	0	1736734215093
2	61	45	1	0	1736734271452
2	61	22	0	0	1736734282399
2	61	47	-1	0	1736734288168
2	61	28	-1	0	1736734296745
2	61	65	0	0	1736734308366
2	61	35	0	0	1736734324321
2	61	33	-1	0	1736734332553
2	61	46	-1	0	1736734337260
2	61	67	-1	0	1736734343201
2	61	36	0	0	1736734350093
2	61	31	-1	0	1736734368362
2	61	57	0	0	1736734371134
2	63	2	0	0	1736734372938
2	61	3	-1	0	1736734374537
2	61	41	-1	0	1736734387671
2	61	12	-1	0	1736734392334
2	64	2	1	0	1736734393768
2	63	30	-1	0	1736734395755
2	64	1	1	0	1736734398471
2	61	21	-1	0	1736734411056
2	61	15	-1	0	1736734413713
2	64	30	0	0	1736734415256
2	61	34	-1	0	1736734417263
2	64	66	-1	0	1736734423223
2	61	17	-1	0	1736734429800
2	61	20	-1	0	1736734450783
2	50	67	-1	0	1736734453181
2	64	12	-1	0	1736734460812
2	61	24	-1	0	1736734468744
2	50	38	-1	0	1736734476023
2	64	9	-1	0	1736734477142
2	77	2	1	0	1736744257200
5	0	6	0	0	1736791252099
5	5	12	0	0	1736794906761
5	12	12	0	0	1736801499036
5	12	10	1	0	1736801502766
5	15	2	1	0	1736804665236
5	15	12	-1	0	1736804671478
5	15	3	1	0	1736804673594
5	15	16	1	0	1736804676447
5	15	7	-1	0	1736804693271
5	15	20	-1	0	1736804715325
5	18	7	-1	0	1736814134524
5	18	13	-1	0	1736814145518
2	78	55	0	0	1736827542965
2	78	61	0	0	1736827549226
2	79	1	1	0	1736879783898
2	79	63	-1	0	1736879805363
2	82	27	-1	0	1736880527211
2	82	72	-1	0	1736880531750
2	82	12	-1	0	1736880903293
2	86	33	-1	0	1736882305940
2	86	38	-1	0	1736882310649
2	86	45	-1	0	1736882356041
2	87	73	0	0	1736916298469
2	87	70	1	0	1736916306691
2	87	58	1	0	1736917229073
2	87	61	-1	0	1736917244873
2	88	12	-1	0	1736994743929
2	89	49	1	0	1737002100519
2	90	30	-1	0	1737041358394
2	90	55	1	0	1737041780794
2	90	29	-1	0	1737041802364
2	91	9	-1	0	1737265431232
2	91	28	-1	0	1737265435080
2	91	22	-1	0	1737265437644
2	91	51	-1	0	1737265443132
2	91	20	0	0	1737265455453
2	91	32	-1	0	1737265463579
2	91	49	0	0	1737265471708
2	91	55	1	0	1737265509026
2	91	50	-1	0	1737265516045
2	91	36	-1	0	1737265527663
11	12	9	-1	0	1739316275290
9	3	1	-1	0	1739371792551
12	0	14	-1	0	1739382213069
12	4	0	1	0	1739454493235
12	4	17	-1	0	1739454503027
12	2	19	-1	0	1739455113344
12	22	19	0	0	1739455582174
12	22	27	1	0	1739455589833
12	29	23	-1	0	1739455597252
12	2	30	0	0	1739456081589
12	22	8	0	0	1739456516825
12	32	4	0	0	1739456527427
12	32	28	1	0	1739456532936
12	18	28	0	0	1739457618018
12	10	7	1	0	1739457631666
12	34	32	0	0	1739458270124
12	9	34	1	0	1739459087850
12	20	3	1	0	1739459771313
12	20	31	-1	0	1739459785657
12	39	28	-1	0	1739459796817
12	27	32	0	0	1739460075874
12	45	17	0	0	1739460899141
12	49	19	-1	0	1739461980281
12	36	14	-1	0	1739462829974
12	3	22	-1	0	1739463402388
12	39	22	-1	0	1739464096234
12	39	35	-1	0	1739464102086
12	41	22	-1	0	1739464102669
12	15	6	0	0	1739464107590
12	49	20	0	0	1739464116533
12	39	31	0	0	1739464117330
12	15	23	-1	0	1739464134369
12	53	17	-1	0	1739464135188
12	16	37	-1	0	1739464811601
12	29	37	-1	0	1739465188860
12	22	22	0	0	1739465194632
12	53	34	0	0	1739465211588
12	53	21	0	0	1739465215714
12	53	36	-1	0	1739465220081
12	29	21	-1	0	1739465232760
12	59	4	-1	0	1739466061663
12	21	38	-1	0	1739467345784
12	60	33	-1	0	1739596015393
12	60	25	-1	0	1739596049612
2	61	42	-1	0	1736734396630
2	63	67	-1	0	1736734401455
2	63	9	-1	0	1736734417136
2	77	55	-1	0	1736744295099
5	0	7	0	0	1736791367939
5	5	8	-1	0	1736794917687
5	0	18	-1	0	1736802170245
5	15	13	-1	0	1736804721761
5	18	5	0	0	1736814139572
5	18	19	-1	0	1736814160317
2	78	63	-1	0	1736827548031
2	78	42	-1	0	1736827561069
2	79	6	1	0	1736879785514
2	82	38	0	0	1736880541961
2	82	68	-1	0	1736880571359
2	82	3	-1	0	1736880913540
2	86	65	-1	0	1736882317495
2	86	70	-1	0	1736882329386
2	87	27	-1	0	1736916366995
2	87	52	1	0	1736917234959
2	87	51	0	0	1736917253991
2	88	56	-1	0	1736994851897
2	89	70	-1	0	1737002104504
2	90	10	-1	0	1737041363349
2	90	68	1	0	1737041795750
2	90	66	0	0	1737041827509
2	90	3	-1	0	1737041834010
2	91	64	0	0	1737265494407
2	91	71	1	0	1737265501168
2	91	11	1	0	1737265530502
2	91	57	-1	0	1737265536034
9	4	2	-1	0	1739378620474
12	0	15	-1	0	1739382223352
9	8	30	0	0	1739454530118
12	2	27	1	0	1739455119576
12	22	23	-1	0	1739455598359
12	30	28	-1	0	1739455626216
12	29	0	-1	0	1739455640996
12	11	30	1	0	1739456097500
12	32	19	1	0	1739456539478
12	32	8	1	0	1739456562011
12	18	19	-1	0	1739457625834
12	10	11	-1	0	1739457637209
12	4	31	-1	0	1739458346738
12	18	34	-1	0	1739459235129
12	13	19	-1	0	1739459776345
12	20	33	-1	0	1739459777945
12	44	1	-1	0	1739460077353
12	44	30	0	0	1739460099546
12	44	31	-1	0	1739460122768
12	45	0	0	0	1739460938488
12	45	2	-1	0	1739460992923
12	51	27	0	0	1739462268004
12	51	11	1	0	1739462299360
12	51	2	-1	0	1739462302308
12	51	23	-1	0	1739462305779
12	51	17	-1	0	1739462320540
12	53	27	0	0	1739462866614
12	10	25	-1	0	1739462878871
12	53	25	-1	0	1739462889292
12	55	22	-1	0	1739463407398
12	17	20	-1	0	1739464096383
12	17	36	-1	0	1739464099511
12	49	22	-1	0	1739464105993
12	39	20	-1	0	1739464106047
12	49	25	-1	0	1739464119278
12	53	20	-1	0	1739464121129
12	21	22	-1	0	1739464132025
12	40	36	-1	0	1739464165607
12	29	35	1	0	1739464167819
12	39	30	1	0	1739464170863
12	15	36	-1	0	1739464175249
12	39	7	0	0	1739464176846
12	40	35	1	0	1739464177642
12	30	25	-1	0	1739464192006
12	57	14	1	0	1739464192302
12	34	37	0	0	1739465014703
12	29	18	-1	0	1739465197750
12	59	25	-1	0	1739466064531
12	2	38	-1	0	1739466072945
12	3	26	-1	0	1739467879652
12	60	14	0	0	1739596024232
2	61	25	-1	0	1736734401177
2	63	66	-1	0	1736734405291
2	61	29	-1	0	1736734406179
2	63	1	1	0	1736734407717
2	63	25	-1	0	1736734412637
2	63	6	-1	0	1736734414737
2	61	43	-1	0	1736734419950
2	61	13	-1	0	1736734422536
2	63	42	-1	0	1736734423307
2	61	48	1	0	1736734425267
2	63	12	-1	0	1736734426948
2	63	68	-1	0	1736734431768
2	61	55	-1	0	1736734435407
2	63	3	-1	0	1736734437288
2	61	52	0	0	1736734439988
2	50	59	-1	0	1736734440571
2	61	11	-1	0	1736734443791
2	61	44	-1	0	1736734445925
2	63	56	0	0	1736734447847
2	50	68	-1	0	1736734448231
2	63	57	0	0	1736734452437
2	64	6	0	0	1736734455402
2	61	37	0	0	1736734457093
2	63	64	-1	0	1736734457157
2	50	66	-1	0	1736734458278
2	63	63	-1	0	1736734459557
2	61	51	-1	0	1736734460752
2	50	63	-1	0	1736734461513
2	64	68	-1	0	1736734465293
2	61	32	0	0	1736734465314
2	63	59	0	0	1736734466797
2	50	52	-1	0	1736734467342
2	64	65	0	0	1736734470412
2	63	60	0	0	1736734473067
2	50	47	-1	0	1736734473148
2	61	49	-1	0	1736734474054
2	64	39	-1	0	1736734474238
2	50	64	-1	0	1736734478698
2	63	58	-1	0	1736734480385
2	64	35	-1	0	1736734480974
2	61	50	-1	0	1736734481605
2	50	61	0	0	1736734482838
2	63	53	-1	0	1736734482917
2	64	59	-1	0	1736734484813
2	64	54	-1	0	1736734487683
2	64	17	-1	0	1736734489931
2	50	23	0	0	1736734493138
2	64	58	0	0	1736734493772
2	63	27	1	0	1736734496156
2	50	12	-1	0	1736734496224
2	64	64	-1	0	1736734498902
2	64	38	-1	0	1736734501132
2	50	58	0	0	1736734502198
2	64	34	-1	0	1736734505301
2	63	35	-1	0	1736734506517
2	50	33	-1	0	1736734507809
2	64	48	0	0	1736734508500
2	63	54	-1	0	1736734511123
2	50	35	-1	0	1736734512910
2	64	67	0	0	1736734512979
2	50	25	-1	0	1736734515639
2	64	27	-1	0	1736734519372
2	64	21	-1	0	1736734522877
2	63	48	0	0	1736734523127
2	61	69	-1	0	1736734528773
2	63	62	0	0	1736734538200
2	63	55	-1	0	1736734541637
2	63	29	-1	0	1736734554308
2	65	2	0	0	1736734566295
2	63	34	-1	0	1736734566525
2	61	70	-1	0	1736734572304
2	65	67	-1	0	1736734572455
2	63	69	-1	0	1736734574175
2	64	60	-1	0	1736734574790
2	63	43	-1	0	1736734576988
2	63	33	-1	0	1736734581078
2	64	28	-1	0	1736734581476
2	63	52	0	0	1736734585390
2	64	70	-1	0	1736734586916
2	65	30	1	0	1736734586954
2	64	69	0	0	1736734589991
2	63	11	0	0	1736734592798
2	64	55	-1	0	1736734593316
2	65	69	1	0	1736734594805
2	63	70	-1	0	1736734596267
2	64	42	-1	0	1736734596527
2	65	1	0	0	1736734600196
2	63	65	-1	0	1736734600667
2	65	54	-1	0	1736734604154
2	63	31	-1	0	1736734605548
2	63	13	-1	0	1736734607897
2	65	70	-1	0	1736734610485
2	63	46	-1	0	1736734612128
2	65	6	0	0	1736734614215
2	63	38	1	0	1736734615709
2	63	23	-1	0	1736734620419
2	65	60	-1	0	1736734620576
2	64	56	-1	0	1736734621380
2	64	63	-1	0	1736734624378
2	65	66	-1	0	1736734625855
2	64	3	-1	0	1736734626310
2	64	32	-1	0	1736734630461
2	65	27	-1	0	1736734630797
2	64	5	-1	0	1736734632366
2	64	61	-1	0	1736734653478
2	64	40	-1	0	1736734658276
2	64	23	-1	0	1736734665671
2	64	41	0	0	1736734675557
2	64	45	-1	0	1736734681326
2	65	25	-1	0	1736734681832
2	65	3	-1	0	1736734685313
2	64	37	-1	0	1736734687728
2	65	38	-1	0	1736734689090
2	64	47	-1	0	1736734694765
2	65	33	0	0	1736734696235
2	65	37	-1	0	1736734700712
2	64	33	-1	0	1736734704040
2	65	51	0	0	1736734706974
2	64	22	0	0	1736734710707
2	65	53	-1	0	1736734712618
2	64	53	-1	0	1736734715064
2	65	55	0	0	1736734723950
2	65	12	-1	0	1736734727968
2	65	58	0	0	1736734737649
2	64	13	-1	0	1736734742562
2	65	46	0	0	1736734746248
2	65	64	-1	0	1736734754238
2	65	28	-1	0	1736734758574
2	64	43	-1	0	1736734764003
2	50	15	-1	0	1736734766915
2	64	31	-1	0	1736734770333
2	50	69	-1	0	1736734771589
2	64	20	-1	0	1736734773512
2	50	13	0	0	1736734775498
2	64	50	-1	0	1736734775919
2	50	27	0	0	1736734781018
2	64	62	0	0	1736734781239
2	64	25	-1	0	1736734783132
2	50	3	-1	0	1736734784970
2	64	46	-1	0	1736734786332
2	64	36	-1	0	1736734789212
2	65	56	-1	0	1736734791549
2	64	44	-1	0	1736734792189
2	65	20	-1	0	1736734798514
2	64	52	0	0	1736734798987
2	64	51	-1	0	1736734801239
2	65	31	-1	0	1736734804810
2	64	11	1	0	1736734805550
2	65	68	0	0	1736734813535
2	62	2	1	0	1736734819245
2	64	15	-1	0	1736734822209
2	64	10	-1	0	1736734824087
2	62	33	-1	0	1736734824150
2	62	35	-1	0	1736734828455
2	64	29	0	0	1736734829838
2	62	1	1	0	1736734830192
2	64	57	0	0	1736734833236
2	62	59	-1	0	1736734836234
2	64	49	-1	0	1736734841857
2	77	58	-1	0	1736744304247
5	0	8	0	0	1736791498691
5	5	7	-1	0	1736794935269
5	5	4	-1	0	1736794946965
5	5	14	1	0	1736794960367
5	0	16	-1	0	1736802171572
5	0	20	-1	0	1736802179016
5	16	21	-1	0	1736807068410
5	16	17	-1	0	1736807085985
5	16	11	-1	0	1736807091248
5	18	18	-1	0	1736814152406
5	18	17	-1	0	1736814171736
5	18	4	-1	0	1736814183767
2	78	15	-1	0	1736827588480
2	79	37	-1	0	1736879795696
2	80	6	1	0	1736879804356
2	82	23	-1	0	1736880547410
2	82	58	1	0	1736880555619
2	82	73	-1	0	1736880560408
2	82	29	1	0	1736880933672
2	82	47	-1	0	1736880962911
2	86	42	-1	0	1736882360116
2	86	59	-1	0	1736882390229
2	86	32	0	0	1736882414850
2	87	20	0	0	1736916373449
2	87	29	-1	0	1736916401492
2	87	13	-1	0	1736917241390
2	87	12	0	0	1736917272928
2	88	13	-1	0	1736994858320
2	88	72	-1	0	1736994872837
2	89	73	-1	0	1737002340034
2	90	50	0	0	1737041402493
2	90	65	1	0	1737041811201
2	91	52	-1	0	1737265533170
9	5	1	-1	0	1739378649942
9	4	1	-1	0	1739378807957
12	0	16	-1	0	1739382229864
12	6	4	-1	0	1739454566000
12	6	17	-1	0	1739454570641
12	2	23	-1	0	1739455135908
12	29	19	-1	0	1739455603003
12	29	27	-1	0	1739455612598
12	29	4	-1	0	1739455630502
12	31	23	-1	0	1739456152940
12	31	29	1	0	1739456168985
12	9	29	1	0	1739456217355
12	26	28	0	0	1739456219025
12	26	19	-1	0	1739456226734
12	26	11	0	0	1739456241946
12	26	23	-1	0	1739456248684
12	26	27	-1	0	1739456252582
12	32	27	1	0	1739456568381
12	9	7	0	0	1739457647572
12	34	31	-1	0	1739458349634
12	34	29	-1	0	1739458401275
12	34	1	0	0	1739459413396
12	13	6	-1	0	1739459789953
12	38	31	1	0	1739459791010
12	10	31	-1	0	1739459795163
12	27	19	-1	0	1739460085318
12	27	8	1	0	1739460115690
12	44	29	-1	0	1739460153860
12	44	6	-1	0	1739460169708
12	45	6	-1	0	1739460960367
12	45	23	-1	0	1739460982979
12	45	4	-1	0	1739461045997
12	51	31	-1	0	1739462274381
12	52	2	-1	0	1739462872802
12	53	1	-1	0	1739462876085
12	5	22	-1	0	1739463409999
12	41	36	1	0	1739464117319
12	39	6	-1	0	1739464121885
12	39	36	-1	0	1739464126346
12	49	14	-1	0	1739464126412
12	29	1	-1	0	1739464150904
12	21	35	-1	0	1739464150996
12	29	14	-1	0	1739464160278
12	15	2	-1	0	1739464168421
12	29	36	-1	0	1739464171447
12	35	27	0	0	1739464174253
12	57	23	-1	0	1739464177586
12	35	36	-1	0	1739464179709
12	35	23	-1	0	1739464194289
12	57	36	-1	0	1739464195132
12	57	20	-1	0	1739464199659
12	35	20	-1	0	1739464213869
12	40	25	-1	0	1739464229401
12	29	13	-1	0	1739464231382
12	15	14	0	0	1739464232087
12	35	22	-1	0	1739464235891
12	53	19	-1	0	1739464238026
12	21	21	-1	0	1739464861483
12	53	8	1	0	1739465241731
12	53	28	1	0	1739465245765
12	59	39	-1	0	1739466147459
12	3	39	-1	0	1739467885479
12	60	35	-1	0	1739596038329
2	64	24	1	0	1736734837391
2	62	47	-1	0	1736734857128
2	62	66	-1	0	1736734860201
2	62	70	1	0	1736734863575
2	62	37	0	0	1736734868601
2	62	30	1	0	1736734873204
2	62	61	-1	0	1736734882316
2	62	63	-1	0	1736734885490
2	62	45	-1	0	1736734894506
2	62	53	-1	0	1736734901675
2	62	68	-1	0	1736734913035
2	62	69	1	0	1736734922048
2	66	6	1	0	1736735101047
2	66	1	1	0	1736735102813
2	66	30	0	0	1736735119249
2	66	64	-1	0	1736735122982
2	66	70	-1	0	1736735125858
2	66	33	-1	0	1736735131640
2	66	63	-1	0	1736735133861
2	66	2	-1	0	1736735142509
2	66	59	-1	0	1736735149607
2	66	42	-1	0	1736735153886
2	66	38	1	0	1736735156608
2	66	20	-1	0	1736735164354
2	66	39	-1	0	1736735169426
2	66	67	-1	0	1736735174960
2	66	66	-1	0	1736735178328
2	66	69	-1	0	1736735181973
2	66	43	-1	0	1736735184458
2	66	15	-1	0	1736735323764
2	66	47	-1	0	1736735327737
2	66	34	-1	0	1736735331557
2	66	40	-1	0	1736735334123
2	66	61	-1	0	1736735336354
2	66	56	-1	0	1736735339290
2	66	48	-1	0	1736735376485
2	66	68	-1	0	1736735380633
2	66	54	-1	0	1736735385247
2	66	37	-1	0	1736735450247
2	66	55	-1	0	1736735459381
2	66	65	-1	0	1736735462325
2	66	13	-1	0	1736735465109
2	66	10	-1	0	1736735468684
2	66	49	0	0	1736735475701
2	66	21	-1	0	1736735479135
2	66	60	-1	0	1736735504088
2	66	22	-1	0	1736735511293
2	66	5	0	0	1736735515564
2	66	25	-1	0	1736735518966
2	66	12	-1	0	1736735521056
2	66	58	-1	0	1736735529718
2	66	9	-1	0	1736735531366
2	66	31	-1	0	1736735537324
2	66	17	-1	0	1736735543121
2	66	35	-1	0	1736735558930
2	66	46	-1	0	1736735563442
2	66	44	-1	0	1736735565494
2	66	32	-1	0	1736735569796
2	66	3	-1	0	1736735615450
2	66	29	-1	0	1736735621484
2	67	1	0	0	1736735626145
2	67	6	0	0	1736735627818
2	66	27	0	0	1736735628235
2	66	45	-1	0	1736735634550
2	66	53	-1	0	1736735640284
2	66	28	-1	0	1736735644094
2	66	11	-1	0	1736735646496
2	67	30	1	0	1736735647502
2	66	51	-1	0	1736735649167
2	67	63	-1	0	1736735650448
2	66	36	-1	0	1736735653216
2	67	2	1	0	1736735653388
2	66	23	-1	0	1736735660612
2	67	70	0	0	1736735660891
2	67	52	0	0	1736735664174
2	66	62	-1	0	1736735664440
2	67	34	-1	0	1736735668043
2	66	50	-1	0	1736735669321
2	68	2	0	0	1736735670848
2	67	66	-1	0	1736735671495
2	67	53	-1	0	1736735673645
2	68	6	1	0	1736735674763
2	67	21	-1	0	1736735678278
2	66	24	-1	0	1736735680023
2	66	41	0	0	1736735686377
2	66	52	0	0	1736735690148
2	66	57	0	0	1736735694142
2	68	31	0	0	1736735703155
2	69	6	0	0	1736735704405
2	68	30	-1	0	1736735713396
2	68	1	1	0	1736735715486
2	68	63	-1	0	1736735719423
2	68	9	-1	0	1736735723417
2	68	12	-1	0	1736735731723
2	68	3	-1	0	1736735735041
2	68	56	0	0	1736735743492
2	68	59	0	0	1736735754053
2	68	69	-1	0	1736735762366
2	68	64	-1	0	1736735768582
2	69	66	0	0	1736735770936
2	68	66	-1	0	1736735771816
2	69	59	-1	0	1736735774716
2	69	70	0	0	1736735778179
2	69	1	0	0	1736735779469
2	68	33	1	0	1736735780197
2	69	2	0	0	1736735781230
2	69	24	0	0	1736735783019
2	69	30	0	0	1736735785183
2	69	34	-1	0	1736735789334
2	68	47	0	0	1736735789854
2	69	69	-1	0	1736735791693
2	68	27	0	0	1736735795576
2	69	64	-1	0	1736735796022
2	69	22	1	0	1736735798888
2	68	34	0	0	1736735802758
2	69	58	1	0	1736735806095
2	69	65	-1	0	1736735809587
2	68	60	-1	0	1736735809690
2	69	33	-1	0	1736735811413
2	69	37	-1	0	1736735816264
2	68	68	-1	0	1736735816577
2	69	5	-1	0	1736735818023
2	68	42	-1	0	1736735821026
2	69	10	-1	0	1736735821044
2	68	38	1	0	1736735824337
2	69	60	-1	0	1736735825972
2	68	11	-1	0	1736735826930
2	69	21	-1	0	1736735827585
2	69	68	-1	0	1736735831708
2	68	21	-1	0	1736735833377
2	69	39	-1	0	1736735836654
2	68	67	1	0	1736735840499
2	69	47	-1	0	1736735842725
2	69	54	-1	0	1736735845324
2	68	20	-1	0	1736735845744
2	69	67	-1	0	1736735848441
2	68	51	0	0	1736735849478
2	69	15	-1	0	1736735850398
2	68	37	-1	0	1736735853883
2	69	55	-1	0	1736735854941
2	69	27	-1	0	1736735857456
2	69	48	0	0	1736735859796
2	68	70	1	0	1736735861370
2	69	35	-1	0	1736735863365
2	69	38	-1	0	1736735865175
2	69	9	-1	0	1736735866477
2	69	62	1	0	1736735871657
2	68	55	0	0	1736735876675
2	69	36	-1	0	1736735877587
2	69	23	-1	0	1736735880377
2	68	13	1	0	1736735881819
2	69	31	-1	0	1736735883999
2	69	44	-1	0	1736735888033
2	68	28	0	0	1736735889122
2	69	53	-1	0	1736735890218
2	69	63	-1	0	1736735893032
2	68	25	-1	0	1736735893076
2	69	61	-1	0	1736735895232
2	69	13	-1	0	1736735901526
2	69	32	-1	0	1736735909295
2	68	10	-1	0	1736735911472
2	68	15	-1	0	1736735914914
2	69	51	-1	0	1736735915208
2	68	58	0	0	1736735953305
2	69	29	-1	0	1736735958240
2	77	17	-1	0	1736744315277
2	77	72	-1	0	1736744340569
5	0	9	0	0	1736791538585
5	0	10	0	0	1736791545645
5	5	15	-1	0	1736794939538
5	5	16	-1	0	1736794973365
5	0	19	-1	0	1736802184856
5	16	16	1	0	1736807072206
5	16	20	0	0	1736807077583
5	16	13	-1	0	1736807089048
5	16	3	-1	0	1736807090433
5	16	2	-1	0	1736807113703
5	16	12	1	0	1736807121903
5	18	8	-1	0	1736814176014
2	78	52	0	0	1736827593393
2	80	1	1	0	1736879802992
2	82	13	1	0	1736880565391
2	82	71	1	0	1736880584458
2	82	56	0	0	1736880613313
2	82	62	0	0	1736880634349
2	82	48	-1	0	1736880942617
2	86	41	1	0	1736882372633
2	86	40	-1	0	1736882396342
2	87	53	-1	0	1736916389064
2	87	33	0	0	1736916421684
2	87	41	-1	0	1736917303880
2	87	48	0	0	1736917320825
2	87	49	-1	0	1736917331906
2	88	28	-1	0	1736994868606
2	88	73	0	0	1736994887793
2	89	44	-1	0	1737002342331
2	89	20	1	0	1737002353121
2	90	44	0	0	1737041409827
2	90	9	-1	0	1737041837069
2	34	5	-1	0	1737406068643
9	5	0	-1	0	1739378652091
12	0	17	-1	0	1739382236501
12	0	18	-1	0	1739382252112
12	6	0	1	0	1739454576000
12	7	17	-1	0	1739454612405
12	5	19	-1	0	1739455143884
12	29	17	-1	0	1739455618556
12	30	27	1	0	1739455642615
12	31	6	1	0	1739456158008
12	32	11	1	0	1739456573277
9	11	34	-1	0	1739456573846
9	8	38	-1	0	1739457711330
12	33	6	0	0	1739457747255
12	33	31	-1	0	1739457756218
12	18	31	-1	0	1739457766563
12	11	31	-1	0	1739457771247
12	33	30	1	0	1739457789286
12	33	17	0	0	1739457796757
9	12	33	-1	0	1739457806432
9	12	35	-1	0	1739457811350
9	12	34	-1	0	1739457816027
12	33	11	1	0	1739457848549
12	4	32	1	0	1739458352417
12	34	11	1	0	1739458360319
12	34	13	1	0	1739459426522
12	22	34	0	0	1739459790062
12	38	17	-1	0	1739459811907
12	27	27	1	0	1739460098524
12	27	7	0	0	1739460107444
12	44	2	-1	0	1739460127168
12	45	19	1	0	1739461077189
12	51	13	-1	0	1739462277704
12	51	33	-1	0	1739462284939
12	54	25	-1	0	1739462896462
12	54	14	-1	0	1739462903941
12	54	27	0	0	1739462921599
12	31	1	1	0	1739462941133
12	54	4	-1	0	1739462953888
12	31	34	1	0	1739462963376
12	31	3	1	0	1739462966498
12	31	11	1	0	1739462979645
12	54	23	-1	0	1739463012067
12	54	34	-1	0	1739463039687
12	54	28	0	0	1739463045110
12	11	22	-1	0	1739463484512
12	15	35	1	0	1739464118635
12	36	36	1	0	1739464127871
12	49	36	-1	0	1739464128595
12	15	27	-1	0	1739464129423
12	39	11	1	0	1739464139605
12	21	25	-1	0	1739464146121
12	17	37	0	0	1739464863023
12	8	18	-1	0	1739465394753
12	5	39	-1	0	1739466161333
12	3	38	-1	0	1739467890156
12	60	21	-1	0	1739596043322
12	60	37	0	0	1739596046772
2	69	42	-1	0	1736735899875
2	68	35	-1	0	1736735902040
2	68	62	0	0	1736735907797
2	69	3	-1	0	1736735911131
2	69	17	-1	0	1736735912843
2	69	45	-1	0	1736735919836
2	68	53	0	0	1736735920611
2	69	20	-1	0	1736735923108
2	68	45	-1	0	1736735930332
2	69	50	0	0	1736735936006
2	69	57	-1	0	1736735941029
2	77	25	-1	0	1736744320344
2	77	68	-1	0	1736744325420
5	0	11	0	0	1736791566273
5	5	3	0	0	1736794941857
5	5	2	0	0	1736794948816
5	13	13	-1	0	1736802790703
5	16	1	1	0	1736807074862
5	16	19	-1	0	1736807083325
5	16	10	-1	0	1736807087883
5	16	9	-1	0	1736807089749
5	16	7	-1	0	1736807102443
5	18	15	0	0	1736814202559
2	78	70	1	0	1736827597099
2	79	46	-1	0	1736879816953
2	82	15	-1	0	1736880591663
2	82	35	1	0	1736880618871
2	82	20	-1	0	1736880947043
2	86	27	1	0	1736882380717
2	86	37	-1	0	1736882385174
2	86	68	-1	0	1736882393604
2	87	44	-1	0	1736916391392
2	87	10	-1	0	1736916406026
2	87	9	-1	0	1736917306275
2	88	69	-1	0	1736994900071
2	89	31	-1	0	1737002347948
2	89	34	-1	0	1737002366162
2	89	10	-1	0	1737002386694
2	90	53	1	0	1737041422753
2	90	56	1	0	1737041445693
2	90	41	-1	0	1737041847444
2	92	1	1	0	1737467377061
2	92	2	1	0	1737467383924
2	92	53	-1	0	1737467388462
2	92	37	-1	0	1737467393322
9	4	0	-1	0	1739378716668
12	0	19	-1	0	1739382266514
12	0	22	-1	0	1739382290352
12	0	23	-1	0	1739382299820
12	0	25	-1	0	1739382318584
12	8	17	-1	0	1739454612443
12	8	0	1	0	1739454626336
12	5	27	-1	0	1739455151008
12	0	29	0	0	1739455658454
12	30	17	-1	0	1739455667976
12	31	17	-1	0	1739456163797
12	32	0	-1	0	1739456583663
12	5	7	1	0	1739457726362
12	34	17	1	0	1739458413045
12	34	28	1	0	1739458428803
12	34	23	-1	0	1739458444835
12	18	13	-1	0	1739459437335
12	21	13	-1	0	1739459463348
12	5	13	-1	0	1739459466342
12	13	8	1	0	1739459795516
12	38	34	-1	0	1739459799717
12	22	1	-1	0	1739459812637
12	36	8	1	0	1739459821253
12	41	34	-1	0	1739459827595
12	10	34	-1	0	1739459829851
12	40	32	1	0	1739459833051
12	7	3	-1	0	1739459847815
12	40	13	-1	0	1739459850728
12	38	28	1	0	1739459865774
12	16	19	-1	0	1739459876404
12	40	29	-1	0	1739459876632
12	7	7	-1	0	1739459884569
12	44	33	-1	0	1739460144311
12	44	3	-1	0	1739460158472
12	46	6	-1	0	1739461093160
12	51	3	-1	0	1739462280886
12	51	0	-1	0	1739462291989
12	53	4	-1	0	1739462900282
12	53	7	-1	0	1739462926707
12	14	25	-1	0	1739462927173
12	53	33	-1	0	1739462934041
12	54	3	-1	0	1739462942482
12	54	11	0	0	1739462961922
12	54	6	-1	0	1739463007338
12	11	1	-1	0	1739463489407
12	34	35	-1	0	1739463496505
12	11	2	-1	0	1739463509575
12	30	14	-1	0	1739464145555
12	39	4	0	0	1739464148414
12	15	11	1	0	1739464151637
12	53	29	0	0	1739464151953
12	21	36	-1	0	1739464154767
12	20	14	-1	0	1739464157921
12	35	4	0	0	1739464158008
12	57	3	1	0	1739464172713
12	30	36	0	0	1739464172991
12	30	35	-1	0	1739464181766
12	56	29	0	0	1739464182841
12	15	33	-1	0	1739464186594
12	20	35	1	0	1739464190653
12	15	19	-1	0	1739464191413
12	21	37	0	0	1739464867467
12	17	21	-1	0	1739464871290
12	34	21	0	0	1739464872103
12	8	35	-1	0	1739465406519
12	8	21	-1	0	1739465435940
12	8	36	-1	0	1739465438445
12	8	25	-1	0	1739465465759
12	2	26	-1	0	1739465529874
12	21	26	-1	0	1739465540410
12	30	39	-1	0	1739466213813
12	0	40	0	0	1739468072686
12	60	42	-1	0	1739596117717
2	69	12	-1	0	1736735925499
2	69	56	-1	0	1736735932121
2	69	40	-1	0	1736735938011
2	68	39	-1	0	1736735941100
2	69	46	-1	0	1736735943849
2	69	41	0	0	1736735948469
2	69	43	0	0	1736735950490
2	69	25	-1	0	1736735952660
2	68	17	-1	0	1736735957573
2	69	49	-1	0	1736735961753
2	68	65	-1	0	1736735962669
2	68	40	1	0	1736735967084
2	69	28	-1	0	1736735970805
2	68	54	-1	0	1736735972243
2	69	52	1	0	1736735975303
2	69	11	-1	0	1736735977487
2	68	61	0	0	1736735977540
2	68	46	-1	0	1736735983646
2	68	24	-1	0	1736735990529
2	68	22	-1	0	1736735996229
2	68	23	-1	0	1736735999263
2	68	48	-1	0	1736736003482
2	68	43	1	0	1736736008033
2	68	50	-1	0	1736736013136
2	68	5	-1	0	1736736015951
2	68	29	0	0	1736736023933
2	68	36	-1	0	1736736029720
2	68	41	0	0	1736736048461
2	68	52	-1	0	1736736053145
2	68	44	0	0	1736736068217
2	68	32	-1	0	1736736073871
2	68	49	-1	0	1736736078230
2	68	57	0	0	1736736082048
2	0	64	-1	0	1736736231796
2	0	69	-1	0	1736736237007
2	0	66	-1	0	1736736239194
2	0	67	-1	0	1736736241580
2	0	68	-1	0	1736736244729
2	0	56	0	0	1736736254926
2	68	71	-1	0	1736736258547
2	0	70	-1	0	1736736260278
2	0	62	-1	0	1736736265478
2	0	71	-1	0	1736736281383
2	0	58	-1	0	1736736285475
2	0	55	-1	0	1736736291382
2	0	60	-1	0	1736736298940
2	0	54	-1	0	1736736302066
2	0	65	-1	0	1736736304521
2	0	53	-1	0	1736736307991
2	0	59	-1	0	1736736311101
2	0	72	-1	0	1736736347316
2	0	61	0	0	1736736363080
2	0	57	0	0	1736736364810
2	70	54	-1	0	1736736427830
2	70	6	0	0	1736736433132
2	70	30	0	0	1736736441687
2	70	2	-1	0	1736736445530
2	71	1	1	0	1736736464002
2	70	71	0	0	1736736464232
2	71	6	1	0	1736736465816
2	70	1	0	0	1736736468770
2	71	13	0	0	1736736468818
2	71	40	-1	0	1736736470572
2	71	71	-1	0	1736736474233
2	71	2	1	0	1736736477438
2	71	38	-1	0	1736736479013
2	71	63	-1	0	1736736480692
2	71	12	-1	0	1736736484082
2	71	30	1	0	1736736485973
2	71	53	-1	0	1736736487773
2	71	65	-1	0	1736736490273
2	72	2	-1	0	1736736532730
2	72	1	1	0	1736736535268
2	72	30	0	0	1736736542283
2	72	38	-1	0	1736736545085
2	72	6	-1	0	1736736547450
2	72	72	-1	0	1736736556048
2	72	60	-1	0	1736736567957
2	72	71	-1	0	1736736581778
2	72	58	-1	0	1736736587847
2	72	10	-1	0	1736736595018
2	72	34	-1	0	1736736601234
2	72	13	-1	0	1736736605433
2	72	64	-1	0	1736736608498
2	72	5	-1	0	1736736617688
2	73	34	-1	0	1736736989043
2	73	64	-1	0	1736736991194
2	73	30	1	0	1736736996366
2	73	1	1	0	1736736998646
2	73	71	1	0	1736737009269
2	73	6	-1	0	1736737012049
2	73	2	-1	0	1736737015918
2	73	69	-1	0	1736737019197
2	73	23	-1	0	1736737022410
2	73	33	1	0	1736737027622
2	73	17	-1	0	1736737030295
2	73	12	-1	0	1736737032947
2	73	37	-1	0	1736737036928
2	73	68	-1	0	1736737040021
2	70	34	0	0	1736737040253
2	73	38	-1	0	1736737041660
2	70	3	-1	0	1736737043614
2	73	51	-1	0	1736737043630
2	73	72	-1	0	1736737047261
2	73	70	-1	0	1736737050834
2	70	72	-1	0	1736737054151
2	73	55	-1	0	1736737056631
2	73	3	-1	0	1736737058664
2	70	42	-1	0	1736737059450
2	70	64	-1	0	1736737063830
2	70	17	-1	0	1736737066526
2	73	39	0	0	1736737079696
2	71	59	-1	0	1736737656485
2	71	39	-1	0	1736737662861
2	71	29	-1	0	1736737666625
2	71	60	0	0	1736737676869
2	71	72	0	0	1736737679322
2	71	64	-1	0	1736737681222
2	71	5	-1	0	1736737682842
2	71	33	0	0	1736737720762
2	71	15	-1	0	1736737722902
2	71	25	-1	0	1736737724820
2	71	51	1	0	1736737728652
2	71	56	-1	0	1736737732700
2	72	69	0	0	1736738095956
2	72	55	-1	0	1736738100005
2	72	67	-1	0	1736738103864
2	72	39	-1	0	1736738108704
2	72	52	0	0	1736738115797
2	72	21	-1	0	1736738120131
2	72	11	-1	0	1736738123651
2	72	15	-1	0	1736738127563
2	72	68	0	0	1736738136858
2	72	40	-1	0	1736738140830
2	72	33	-1	0	1736738145663
2	72	45	-1	0	1736738154127
2	72	54	-1	0	1736738159474
2	72	61	-1	0	1736738161414
2	72	3	-1	0	1736738172491
2	72	66	-1	0	1736738175772
2	72	70	-1	0	1736738179572
2	72	29	0	0	1736738188802
2	72	51	-1	0	1736738192078
2	72	31	-1	0	1736738196568
2	72	50	-1	0	1736738200567
2	72	62	0	0	1736738206317
2	72	43	-1	0	1736738208979
2	72	12	-1	0	1736738214523
2	72	47	0	0	1736738224309
2	72	23	-1	0	1736738227777
2	72	28	-1	0	1736738234281
2	72	37	-1	0	1736738239489
2	72	17	-1	0	1736738242422
2	72	32	-1	0	1736738250658
2	72	44	-1	0	1736738252991
2	72	22	-1	0	1736738260853
2	72	35	-1	0	1736738273362
2	72	59	-1	0	1736738282779
2	72	20	-1	0	1736738284741
2	72	53	-1	0	1736738293675
2	72	48	-1	0	1736738301415
2	72	65	-1	0	1736738329923
2	59	70	1	0	1736744748847
5	0	12	0	0	1736791579161
5	5	5	1	0	1736794963053
5	13	15	-1	0	1736802795159
5	16	6	1	0	1736807108789
5	16	5	1	0	1736807138379
5	19	16	0	0	1736814214374
2	78	37	-1	0	1736827617830
2	79	60	-1	0	1736879839875
2	82	39	-1	0	1736880598882
2	82	22	1	0	1736880952827
2	86	71	0	0	1736882422191
2	87	39	-1	0	1736916436786
2	87	25	0	0	1736917314076
2	87	50	-1	0	1736917326340
2	87	62	0	0	1736917345348
2	88	47	0	0	1736994915127
2	89	63	-1	0	1737002368972
2	89	33	-1	0	1737002398998
2	89	48	1	0	1737002417962
2	90	5	-1	0	1737041433896
2	90	22	1	0	1737041857958
2	92	66	-1	0	1737467379487
2	92	43	-1	0	1737467386287
2	92	27	-1	0	1737467396477
2	92	40	-1	0	1737467422421
2	92	35	-1	0	1737467433728
2	92	28	-1	0	1737467447698
2	92	34	-1	0	1737467452204
2	92	33	-1	0	1737467463652
2	92	72	-1	0	1737467470124
2	92	32	-1	0	1737467482780
2	92	39	1	0	1737467501832
2	92	47	-1	0	1737467514468
2	92	58	1	0	1737467531005
2	92	44	-1	0	1737467538190
9	6	3	-1	0	1739381298585
12	0	20	-1	0	1739382273234
12	8	4	-1	0	1739454619252
12	9	4	-1	0	1739454624492
12	12	0	1	0	1739454663348
12	13	17	-1	0	1739454664841
12	15	0	-1	0	1739454699535
12	17	0	-1	0	1739454708596
12	5	23	-1	0	1739455154523
12	11	29	1	0	1739455676721
12	31	30	1	0	1739456174574
12	31	28	1	0	1739456178873
9	11	35	-1	0	1739456650431
12	0	31	0	0	1739457730950
12	5	31	-1	0	1739457754474
12	33	7	1	0	1739457782079
12	34	6	-1	0	1739458478027
12	21	30	1	0	1739459445048
12	22	13	-1	0	1739459796662
12	20	32	1	0	1739459797107
12	38	3	-1	0	1739459802568
12	7	1	-1	0	1739459803237
12	38	33	-1	0	1739459805058
12	22	33	-1	0	1739459806581
12	7	34	1	0	1739459814205
12	7	29	1	0	1739459819266
12	10	1	-1	0	1739459820952
12	38	1	-1	0	1739459820992
12	7	2	-1	0	1739459832115
12	17	32	0	0	1739459832261
12	10	3	-1	0	1739459832567
12	44	19	0	0	1739460176654
12	44	11	-1	0	1739460217775
12	44	32	1	0	1739460230622
12	45	32	0	0	1739461110840
12	51	1	-1	0	1739462311632
12	53	11	1	0	1739462907267
12	31	2	-1	0	1739462918813
12	41	14	-1	0	1739462923387
12	54	2	0	0	1739462923973
12	31	14	1	0	1739462925664
12	34	22	-1	0	1739463490144
12	56	34	-1	0	1739464193920
12	29	7	0	0	1739464198876
12	56	19	-1	0	1739464199159
12	40	14	0	0	1739464199943
12	58	29	-1	0	1739464201205
12	15	3	1	0	1739464203995
12	57	25	-1	0	1739464204823
12	53	22	-1	0	1739464206432
12	10	36	-1	0	1739464228965
12	5	18	-1	0	1739464906830
12	8	22	1	0	1739465416847
12	30	38	1	0	1739466219750
12	5	40	-1	0	1739468106962
12	60	38	1	0	1739596121907
2	72	27	1	0	1736738311443
2	72	56	-1	0	1736738678722
2	72	25	-1	0	1736738681756
2	72	63	-1	0	1736738689261
2	72	42	-1	0	1736738694331
2	72	49	0	0	1736738703951
2	72	9	-1	0	1736738706699
2	72	46	-1	0	1736738710826
2	72	36	0	0	1736738715767
2	72	41	0	0	1736738723967
2	72	57	0	0	1736738728239
2	72	24	0	0	1736738738305
2	61	72	1	0	1736738751054
2	61	71	-1	0	1736738757202
2	74	68	-1	0	1736738953592
2	74	72	-1	0	1736738962633
2	74	2	-1	0	1736738966226
2	74	30	1	0	1736738976247
2	74	21	-1	0	1736738981446
2	74	64	-1	0	1736738988746
2	74	1	1	0	1736738990946
2	74	48	0	0	1736738997566
2	74	6	1	0	1736739000306
2	74	63	-1	0	1736739004206
2	74	56	-1	0	1736739021986
2	74	71	1	0	1736739035652
2	74	36	-1	0	1736739040568
2	75	1	-1	0	1736740161609
2	75	2	0	0	1736740166911
2	75	6	1	0	1736740191399
2	75	30	0	0	1736740199526
2	75	72	0	0	1736740210094
2	75	35	-1	0	1736740215899
2	75	31	-1	0	1736740220531
2	75	65	-1	0	1736740228399
2	75	17	-1	0	1736740231549
2	75	25	-1	0	1736740234133
2	75	66	-1	0	1736740238784
2	75	47	-1	0	1736740247760
2	75	9	-1	0	1736740251205
2	75	71	0	0	1736740267207
2	75	68	-1	0	1736740272844
2	75	38	0	0	1736740279977
2	75	64	0	0	1736740283813
2	75	58	-1	0	1736740289976
2	75	33	-1	0	1736740295508
2	75	67	0	0	1736740302063
2	75	13	-1	0	1736740304664
2	75	3	-1	0	1736740308396
2	75	69	-1	0	1736740312083
2	75	15	-1	0	1736740318134
2	75	70	-1	0	1736740321834
2	75	63	-1	0	1736740326519
2	75	34	-1	0	1736740332078
2	75	59	-1	0	1736740352572
2	75	12	0	0	1736740356456
2	75	22	1	0	1736740368422
2	75	28	-1	0	1736740375392
2	75	37	-1	0	1736740386329
2	75	53	-1	0	1736740390047
2	75	23	0	0	1736740395478
2	75	54	-1	0	1736740400830
2	75	29	-1	0	1736740407586
2	75	48	-1	0	1736740415018
2	75	46	-1	0	1736740419749
2	59	72	-1	0	1736744752468
5	1	12	-1	0	1736794349435
5	1	9	1	0	1736794351658
5	1	6	-1	0	1736794358618
5	6	14	0	0	1736795450871
5	13	5	-1	0	1736802797614
5	13	7	-1	0	1736802814428
5	16	14	-1	0	1736807140195
5	20	22	0	0	1736814454272
5	20	10	0	0	1736814462281
5	20	2	0	0	1736814472664
5	20	13	1	0	1736814517846
5	20	6	-1	0	1736814525626
5	20	4	-1	0	1736814566206
5	20	15	0	0	1736814617797
5	20	14	-1	0	1736814656693
2	78	35	0	0	1736827626977
2	79	73	0	0	1736879843992
2	79	39	-1	0	1736879853511
2	79	12	-1	0	1736879880014
2	81	66	-1	0	1736879889121
2	79	10	0	0	1736879894415
2	81	5	0	0	1736879899651
2	82	46	1	0	1736880653412
2	82	41	-1	0	1736880994452
2	86	53	-1	0	1736882424951
2	87	67	-1	0	1736916454103
2	87	24	0	0	1736917357769
2	88	35	-1	0	1736994930956
2	89	58	-1	0	1737002374228
2	89	27	1	0	1737002384514
2	89	17	-1	0	1737002389645
2	89	37	-1	0	1737002407768
2	89	40	-1	0	1737002442634
2	89	68	0	0	1737002448686
2	89	67	-1	0	1737002481947
2	90	27	-1	0	1737041456810
2	90	47	1	0	1737041883879
2	92	6	1	0	1737467381036
2	92	70	-1	0	1737467401871
2	92	46	-1	0	1737467408804
2	92	55	-1	0	1737467417086
2	92	13	-1	0	1737467438203
2	92	5	-1	0	1737467442816
2	92	42	-1	0	1737467454989
2	92	12	-1	0	1737467457500
2	92	63	-1	0	1737467474089
2	92	30	-1	0	1737467484873
9	6	4	-1	0	1739381326667
12	0	21	-1	0	1739382283575
12	0	24	-1	0	1739382306539
12	7	4	-1	0	1739454620418
12	2	17	-1	0	1739454624567
12	7	0	1	0	1739454627130
12	9	0	1	0	1739454631099
12	0	28	0	0	1739455245015
12	30	29	-1	0	1739455683232
12	31	27	0	0	1739456184290
12	26	29	1	0	1739456216006
12	9	8	1	0	1739456228213
12	30	30	1	0	1739456713804
12	33	29	-1	0	1739457739235
12	33	23	-1	0	1739457761672
12	2	31	-1	0	1739457763897
12	0	33	0	0	1739458495034
12	21	1	-1	0	1739459449824
12	21	2	-1	0	1739459454984
12	21	3	-1	0	1739459457624
12	5	1	-1	0	1739459469881
12	21	7	-1	0	1739459471347
12	21	33	0	0	1739459493929
12	10	33	1	0	1739459815069
12	7	33	-1	0	1739459829265
12	41	33	-1	0	1739459833058
12	41	13	-1	0	1739459837207
12	17	3	1	0	1739459839887
12	38	32	0	0	1739459842596
12	41	31	-1	0	1739459844353
12	44	4	-1	0	1739460195941
12	8	1	-1	0	1739460230733
12	44	7	0	0	1739460252639
12	8	30	1	0	1739460257235
12	45	27	-1	0	1739461138743
12	45	11	0	0	1739461168736
12	51	19	-1	0	1739462315231
12	51	28	0	0	1739462331385
12	54	0	1	0	1739462912140
12	11	34	-1	0	1739463533477
12	58	25	-1	0	1739464197005
12	40	22	-1	0	1739464223306
12	29	34	1	0	1739464224050
12	15	31	-1	0	1739464238117
\.


--
-- Data for Name: worker_tasks; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.worker_tasks (created, math_env, attempts, task_data, task_type, task_bucket, finished_time) FROM stdin;
\.


--
-- Data for Name: xid_whitelist; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.xid_whitelist (owner, xid, created) FROM stdin;
\.


--
-- Data for Name: xids; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.xids (uid, owner, xid, x_profile_image_url, x_name, x_email, created, modified) FROM stdin;
132	122	undefined	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user3.jpeg	Participant	\N	1739308084584	1739308084584
133	122	0ba133fb-3c21-44cd-8a43-8ed0ab6ddda5	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user3.jpeg	Participant	\N	1739308680107	1739308680107
134	122	7b3e3f9d-c934-4727-afbe-60f28d2ccd7b	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/fac2.jpeg	Facilitator 1	\N	1739313565116	1739313565116
135	122	6c4ac217-648e-4206-96d3-bb8789cba4e4	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user3.jpeg	Anonymous	\N	1739316193579	1739316193579
136	122	d5f9587b-eb4c-494c-a524-a19193501682	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/fac2.jpeg	Speaker 1	\N	1739316970578	1739316970578
137	122	ce4f2e72-97d9-4303-8fc5-874b7f6a9e6b	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/fac2.jpeg	Speaker 1	\N	1739317069028	1739317069028
170	122	ebe9fb0b-b046-4bcb-8a85-1a732a715145	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/fac2.jpeg	Speaker 1	\N	1739317205017	1739317205017
171	122	f8cc8f87-bbbe-43b7-8740-8c2eb3a2d484	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user6.jpeg	Anonymous	\N	1739318781037	1739318781037
173	122	4928b2f6-15ea-4623-94d5-6680b913bc45	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user2.jpeg	Anonymous	\N	1739362870253	1739362870253
174	122	f69ed4f0-db5f-47bc-bfad-b3f77c205ad0	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user7.jpeg	Anonymous	\N	1739363141558	1739363141558
176	122	f98171c0-6ab7-4fdf-9c87-23bba350a169	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/fac2.jpeg	Speaker 1	\N	1739378620274	1739378620274
181	122	0bcfcea6-e612-46fb-ba4b-2ffbc00235e9	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user2.jpeg	Anonymous	\N	1739454339151	1739454339151
183	122	030ae34e-8658-4517-99db-0c297ca651ab	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user4.jpeg	Anonymous	\N	1739454392194	1739454392194
184	122	b939cdb9-faa9-49a5-b1ac-726aeb6174c0	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user4.jpeg	Anonymous	\N	1739454485136	1739454485136
185	122	452d2f4e-7434-4bc0-bb67-fa47b96d9e6e	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user7.jpeg	Anonymous	\N	1739454492646	1739454492646
186	122	35c6ca06-2e7a-41ba-8f33-41ce79fa09e6	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user4.jpeg	Anonymous	\N	1739454565968	1739454565968
187	122	9c4f85ad-8a2c-41fc-aaed-3548a5c431e8	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user2.jpeg	Anonymous	\N	1739454612375	1739454612375
188	122	7eaa9350-11a7-4aac-85b8-8e2c1b509e8a	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user7.jpeg	Anonymous	\N	1739454612431	1739454612431
189	122	2c7dfd50-cfb4-4f9c-8b21-ad78a1eb0ff7	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user4.jpeg	Anonymous	\N	1739454624472	1739454624472
190	122	a1e65b25-337d-4d2b-af3e-cc2257da4a1a	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user5.jpeg	Anonymous	\N	1739454654356	1739454654356
191	122	92908b97-81d7-4c0b-8f6e-67de1993e429	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user1.jpeg	Anonymous	\N	1739454656442	1739454656442
192	122	81e56552-f431-44c6-a23c-8aa7c0d051dc	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user2.jpeg	Anonymous	\N	1739454663333	1739454663333
193	122	5e3f6547-e8ec-46bf-8d38-14d80e2a6e54	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user6.jpeg	Anonymous	\N	1739454664830	1739454664830
194	122	aa1232eb-f93a-4dcf-b2a7-933a3db65f18	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user1.jpeg	Anonymous	\N	1739454686942	1739454686942
195	122	6efafc2f-e31b-438f-b520-d95f79642f66	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user1.jpeg	Anonymous	\N	1739454690559	1739454690559
196	122	f8f1c1bd-b47f-43d4-9629-d2c95ff01cd7	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user7.jpeg	Anonymous	\N	1739454692395	1739454692395
197	122	3137f512-59d9-4627-881a-2903e12f8758	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user2.jpeg	Anonymous	\N	1739454699289	1739454699289
198	122	1da61af9-df92-4377-858e-e85d623648d0	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user3.jpeg	Anonymous	\N	1739454704506	1739454704506
199	122	8e95cbc1-00fc-472b-aeb0-10d75351e80b	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user7.jpeg	Anonymous	\N	1739454706642	1739454706642
200	122	5f1c4dc6-f1d6-45a1-8e75-91741997da28	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user6.jpeg	Anonymous	\N	1739454718127	1739454718127
201	122	0d75710c-9671-40db-ba74-ad979a484cce	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user4.jpeg	Anonymous	\N	1739454720099	1739454720099
202	122	79921237-3899-4877-99d2-62fc33bb4bbc	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user2.jpeg	Anonymous	\N	1739454726508	1739454726508
203	122	e0109ba5-056f-4af6-bad6-54ccff95d415	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user5.jpeg	Anonymous	\N	1739454734184	1739454734184
204	122	7b63c9c5-fe9b-44d6-bdb7-8d0c567f5342	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user6.jpeg	Anonymous	\N	1739454734748	1739454734748
205	122	4e5116de-aaf7-48aa-9b1f-adf0e0c06e2c	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user5.jpeg	Anonymous	\N	1739454774968	1739454774968
206	122	7c0295e6-4c06-4904-b7d5-571fdebea7d4	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user3.jpeg	Anonymous	\N	1739454852974	1739454852974
207	122	47cd06ea-ae75-424b-bc27-ce3410f430f4	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user4.jpeg	Anonymous	\N	1739454891742	1739454891742
208	122	34091f40-0822-4246-afa3-144e1adc0091	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user4.jpeg	Anonymous	\N	1739454999006	1739454999006
210	122	55e229f3-4b57-4930-a7ed-844910104ab3	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user1.jpeg	Anonymous	\N	1739455585247	1739455585247
211	122	4b7326b8-6d23-48b2-b416-df2a274caeb8	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user1.jpeg	Anonymous	\N	1739455626192	1739455626192
212	122	cf56906c-9eeb-447a-9b32-504fcbaed5c4	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user4.jpeg	Anonymous	\N	1739456152920	1739456152920
213	122	cd33f318-6c78-41d9-a9ee-b59a19f53c8d	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user7.jpeg	Anonymous	\N	1739456444958	1739456444958
214	122	74887b4d-b07b-4cb4-bd64-5801f0d30abf	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user3.jpeg	Anonymous	\N	1739457739214	1739457739214
216	122	b7d387c3-15e3-4416-b3e5-c137ccfcc4fe	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user6.jpeg	Anonymous	\N	1739458134516	1739458134516
217	122	5dafe606-d24a-42fc-b1ae-0075993b8ce4	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user5.jpeg	Anonymous	\N	1739459642290	1739459642290
218	122	13944e6a-e2d1-4eec-873e-2fdc643b67f1	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user1.jpeg	Anonymous	\N	1739459655458	1739459655458
219	122	ebf516d7-d20b-4fef-a36b-442b049971b3	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user7.jpeg	Anonymous	\N	1739459703332	1739459703332
220	122	fefd4257-0de6-4ef0-9695-560ec0757fca	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user5.jpeg	Anonymous	\N	1739459790984	1739459790984
221	122	350313a1-fa8b-478d-b682-f979e2a6b8de	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user4.jpeg	Anonymous	\N	1739459796783	1739459796783
222	122	1bb559c5-0465-4ec0-8ef5-49908042d117	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user3.jpeg	Anonymous	\N	1739459805353	1739459805353
223	122	1c1ff7c6-71cf-4fc9-a333-60f8e4fc482c	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user2.jpeg	Anonymous	\N	1739459827575	1739459827575
224	122	a29c7263-ddd3-433b-a1fb-92af1aac76c2	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user2.jpeg	Anonymous	\N	1739459887789	1739459887789
225	122	9cc3a0ef-8a50-4a8a-a0d3-7b32dbc9deeb	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user3.jpeg	Anonymous	\N	1739459904665	1739459904665
226	122	b7397eb6-5247-4bce-9058-f92443ddcf71	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user7.jpeg	Anonymous	\N	1739460044130	1739460044130
227	122	381822dd-4139-4c1f-8dea-7ecba318c59b	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user3.jpeg	Anonymous	\N	1739460441666	1739460441666
229	122	0ef851b4-9d53-4694-b4bc-5f8b22f73cb2	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user7.jpeg	Anonymous	\N	1739461093132	1739461093132
230	122	64c735c5-eaeb-4d8d-83b2-b6e7723e5425	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user4.jpeg	Anonymous	\N	1739461833733	1739461833733
231	122	fe9b739b-8dae-43e7-b04f-2192c6a473be	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user1.jpeg	Anonymous	\N	1739461875381	1739461875381
232	122	98b85458-c3a2-40ab-8281-88b5f47ffeeb	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user1.jpeg	Anonymous	\N	1739461878721	1739461878721
233	122	55c9736c-e90f-4628-aa50-745de8d77c0d	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user5.jpeg	Anonymous	\N	1739461906743	1739461906743
234	122	ccfa701a-a49c-4fa8-83d9-1fcdecacfafd	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user3.jpeg	Anonymous	\N	1739462267971	1739462267971
236	122	0fc13665-3364-4aec-ad7b-a646810b0482	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user5.jpeg	Anonymous	\N	1739462818948	1739462818948
237	122	c2f60f28-6d1e-4729-ad72-f989a8911f18	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user7.jpeg	Anonymous	\N	1739462850000	1739462850000
238	122	e3223bd4-ae76-455c-8b75-66b848557cb1	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user2.jpeg	Anonymous	\N	1739462896442	1739462896442
239	122	86408954-a538-4669-9310-980e3055e6f9	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user5.jpeg	Anonymous	\N	1739463386537	1739463386537
240	122	dc493105-e662-4d9a-9e55-71f6cef4e416	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user5.jpeg	Anonymous	\N	1739463945487	1739463945487
241	122	f11f3162-01ad-4497-b354-0add9f979a0a	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user6.jpeg	Anonymous	\N	1739464168416	1739464168416
242	122	b2d69188-483b-41e4-aa3a-e5381740f48f	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user3.jpeg	Anonymous	\N	1739464196972	1739464196972
243	122	00ce2571-b23b-4975-bdb1-adbfbd882b52	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user6.jpeg	Anonymous	\N	1739465962076	1739465962076
245	122	94bcb277-e677-4518-8eb3-9b71752780ec	https://crownshy.s3.eu-west-2.amazonaws.com/paris_ai_icons/user5.jpeg	Anonymous	\N	1739595941183	1739595941183
\.


--
-- Data for Name: zinvites; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.zinvites (zid, zinvite, created) FROM stdin;
1	8wm9undh8r	1736635873506
2	9huj328rfh	1736644696160
3	65uhuaa4hk	1736648068908
4	5d7tssersj	1736754529914
5	4hnrxyx4kr	1736787182247
6	6ncab4jemw	1736958249638
7	6dnmmwcxkr	1737721918151
8	2djm9xjy3s	1737739910182
9	8p5dmt5dut	1739182649070
10	9fsnefnrba	1739197860114
11	6hf7mhtmp3	1739296865035
12	9pjrcuzmfh	1739381669832
13	868udn3b4b	1739539462334
\.


--
-- Name: contexts_context_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.contexts_context_id_seq', 1, false);


--
-- Name: conversations_zid_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.conversations_zid_seq', 13, true);


--
-- Name: courses_course_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.courses_course_id_seq', 1, false);


--
-- Name: participant_metadata_answers_pmaid_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.participant_metadata_answers_pmaid_seq', 1, false);


--
-- Name: participant_metadata_questions_pmqid_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.participant_metadata_questions_pmqid_seq', 1, false);


--
-- Name: reports_rid_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.reports_rid_seq', 17, true);


--
-- Name: users_uid_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.users_uid_seq', 246, true);


--
-- Name: apikeysndvweifu apikeysndvweifu_apikey_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.apikeysndvweifu
    ADD CONSTRAINT apikeysndvweifu_apikey_key UNIQUE (apikey);


--
-- Name: auth_tokens auth_tokens_token_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.auth_tokens
    ADD CONSTRAINT auth_tokens_token_key UNIQUE (token);


--
-- Name: beta beta_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.beta
    ADD CONSTRAINT beta_email_key UNIQUE (email);


--
-- Name: comment_translations comment_translations_zid_tid_src_lang_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.comment_translations
    ADD CONSTRAINT comment_translations_zid_tid_src_lang_key UNIQUE (zid, tid, src, lang);


--
-- Name: comments comments_zid_tid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.comments
    ADD CONSTRAINT comments_zid_tid_key UNIQUE (zid, tid);


--
-- Name: comments comments_zid_txt_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.comments
    ADD CONSTRAINT comments_zid_txt_key UNIQUE (zid, txt);


--
-- Name: conversation_translations conversation_translations_zid_src_lang_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.conversation_translations
    ADD CONSTRAINT conversation_translations_zid_src_lang_key UNIQUE (zid, src, lang);


--
-- Name: conversations conversations_zid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.conversations
    ADD CONSTRAINT conversations_zid_key UNIQUE (zid);


--
-- Name: courses courses_course_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.courses
    ADD CONSTRAINT courses_course_id_key UNIQUE (course_id);


--
-- Name: courses courses_course_invite_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.courses
    ADD CONSTRAINT courses_course_invite_key UNIQUE (course_invite);


--
-- Name: demographic_data demographic_data_uid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.demographic_data
    ADD CONSTRAINT demographic_data_uid_key UNIQUE (uid);


--
-- Name: einvites einvites_einvite_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.einvites
    ADD CONSTRAINT einvites_einvite_key UNIQUE (einvite);


--
-- Name: email_validations email_validations_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.email_validations
    ADD CONSTRAINT email_validations_email_key UNIQUE (email);


--
-- Name: facebook_users facebook_users_fb_user_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.facebook_users
    ADD CONSTRAINT facebook_users_fb_user_id_key UNIQUE (fb_user_id);


--
-- Name: facebook_users facebook_users_uid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.facebook_users
    ADD CONSTRAINT facebook_users_uid_key UNIQUE (uid);


--
-- Name: jianiuevyew jianiuevyew_uid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.jianiuevyew
    ADD CONSTRAINT jianiuevyew_uid_key UNIQUE (uid);


--
-- Name: math_bidtopid math_bidtopid_zid_math_env_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.math_bidtopid
    ADD CONSTRAINT math_bidtopid_zid_math_env_key UNIQUE (zid, math_env);


--
-- Name: math_cache math_cache_zid_math_env_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.math_cache
    ADD CONSTRAINT math_cache_zid_math_env_key UNIQUE (zid, math_env);


--
-- Name: math_exportstatus math_exportstatus_zid_math_env_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.math_exportstatus
    ADD CONSTRAINT math_exportstatus_zid_math_env_key UNIQUE (zid, math_env);


--
-- Name: math_main math_main_zid_math_env_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.math_main
    ADD CONSTRAINT math_main_zid_math_env_key UNIQUE (zid, math_env);


--
-- Name: math_profile math_profile_zid_math_env_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.math_profile
    ADD CONSTRAINT math_profile_zid_math_env_key UNIQUE (zid, math_env);


--
-- Name: math_ptptstats math_ptptstats_zid_math_env_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.math_ptptstats
    ADD CONSTRAINT math_ptptstats_zid_math_env_key UNIQUE (zid, math_env);


--
-- Name: math_report_correlationmatrix math_report_correlationmatrix_rid_math_env_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.math_report_correlationmatrix
    ADD CONSTRAINT math_report_correlationmatrix_rid_math_env_key UNIQUE (rid, math_env);


--
-- Name: math_ticks math_ticks_zid_math_env_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.math_ticks
    ADD CONSTRAINT math_ticks_zid_math_env_key UNIQUE (zid, math_env);


--
-- Name: notification_tasks notification_tasks_zid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.notification_tasks
    ADD CONSTRAINT notification_tasks_zid_key UNIQUE (zid);


--
-- Name: oinvites oinvites_oinvite_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.oinvites
    ADD CONSTRAINT oinvites_oinvite_key UNIQUE (oinvite);


--
-- Name: page_ids page_ids_site_id_page_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.page_ids
    ADD CONSTRAINT page_ids_site_id_page_id_key UNIQUE (site_id, page_id);


--
-- Name: participant_locations participant_locations_zid_uid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participant_locations
    ADD CONSTRAINT participant_locations_zid_uid_key UNIQUE (zid, uid);


--
-- Name: participant_metadata_answers participant_metadata_answers_pmaid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participant_metadata_answers
    ADD CONSTRAINT participant_metadata_answers_pmaid_key UNIQUE (pmaid);


--
-- Name: participant_metadata_answers participant_metadata_answers_pmqid_zid_value_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participant_metadata_answers
    ADD CONSTRAINT participant_metadata_answers_pmqid_zid_value_key UNIQUE (pmqid, zid, value);


--
-- Name: participant_metadata_choices participant_metadata_choices_zid_pid_pmqid_pmaid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participant_metadata_choices
    ADD CONSTRAINT participant_metadata_choices_zid_pid_pmqid_pmaid_key UNIQUE (zid, pid, pmqid, pmaid);


--
-- Name: participant_metadata_questions participant_metadata_questions_pmqid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participant_metadata_questions
    ADD CONSTRAINT participant_metadata_questions_pmqid_key UNIQUE (pmqid);


--
-- Name: participant_metadata_questions participant_metadata_questions_zid_key_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participant_metadata_questions
    ADD CONSTRAINT participant_metadata_questions_zid_key_key UNIQUE (zid, key);


--
-- Name: participants_extended participants_extended_zid_uid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participants_extended
    ADD CONSTRAINT participants_extended_zid_uid_key UNIQUE (zid, uid);


--
-- Name: participants participants_zid_pid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participants
    ADD CONSTRAINT participants_zid_pid_key UNIQUE (zid, pid);


--
-- Name: participants participants_zid_uid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participants
    ADD CONSTRAINT participants_zid_uid_key UNIQUE (zid, uid);


--
-- Name: pwreset_tokens password_reset_tokens_pwresettoken_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.pwreset_tokens
    ADD CONSTRAINT password_reset_tokens_pwresettoken_key UNIQUE (token);


--
-- Name: permanentcookiezidjoins permanentcookiezidjoins_zid_cookie_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.permanentcookiezidjoins
    ADD CONSTRAINT permanentcookiezidjoins_zid_cookie_key UNIQUE (zid, cookie);


--
-- Name: report_comment_selections report_comment_selections_rid_tid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.report_comment_selections
    ADD CONSTRAINT report_comment_selections_rid_tid_key UNIQUE (rid, tid);


--
-- Name: reports reports_report_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.reports
    ADD CONSTRAINT reports_report_id_key UNIQUE (report_id);


--
-- Name: reports reports_rid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.reports
    ADD CONSTRAINT reports_rid_key UNIQUE (rid);


--
-- Name: suzinvites suzinvites_suzinvite_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.suzinvites
    ADD CONSTRAINT suzinvites_suzinvite_key UNIQUE (suzinvite);


--
-- Name: twitter_users twitter_users_twitter_user_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.twitter_users
    ADD CONSTRAINT twitter_users_twitter_user_id_key UNIQUE (twitter_user_id);


--
-- Name: twitter_users twitter_users_uid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.twitter_users
    ADD CONSTRAINT twitter_users_uid_key UNIQUE (uid);


--
-- Name: upvotes upvotes_uid_zid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.upvotes
    ADD CONSTRAINT upvotes_uid_zid_key UNIQUE (uid, zid);


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- Name: users users_uid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_uid_key UNIQUE (uid);


--
-- Name: votes_latest_unique votes_latest_unique_zid_pid_tid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.votes_latest_unique
    ADD CONSTRAINT votes_latest_unique_zid_pid_tid_key UNIQUE (zid, pid, tid);


--
-- Name: xid_whitelist xid_whitelist_owner_xid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.xid_whitelist
    ADD CONSTRAINT xid_whitelist_owner_xid_key UNIQUE (owner, xid);


--
-- Name: xids xids_owner_xid_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.xids
    ADD CONSTRAINT xids_owner_xid_key UNIQUE (owner, xid);


--
-- Name: zinvites zinvites_zinvite_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.zinvites
    ADD CONSTRAINT zinvites_zinvite_key UNIQUE (zinvite);


--
-- Name: apikeysndvweifu_apikey_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX apikeysndvweifu_apikey_idx ON public.apikeysndvweifu USING btree (apikey);


--
-- Name: apikeysndvweifu_uid_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX apikeysndvweifu_uid_idx ON public.apikeysndvweifu USING btree (uid);


--
-- Name: comment_translations_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX comment_translations_idx ON public.comment_translations USING btree (zid, tid);


--
-- Name: comments_zid_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX comments_zid_idx ON public.comments USING btree (zid);


--
-- Name: conversation_translations_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX conversation_translations_idx ON public.conversation_translations USING btree (zid);


--
-- Name: conversations_owner_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX conversations_owner_idx ON public.conversations USING btree (owner);


--
-- Name: course_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX course_id_idx ON public.courses USING btree (course_id);


--
-- Name: main_main_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX main_main_idx ON public.math_main USING btree (zid);


--
-- Name: main_profile_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX main_profile_idx ON public.math_profile USING btree (zid);


--
-- Name: math_bidtopid_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX math_bidtopid_idx ON public.math_bidtopid USING btree (zid);


--
-- Name: math_cache_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX math_cache_idx ON public.math_cache USING btree (zid);


--
-- Name: math_exportstatus_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX math_exportstatus_idx ON public.math_exportstatus USING btree (zid);


--
-- Name: math_math_report_correlationmatrix_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX math_math_report_correlationmatrix_idx ON public.math_report_correlationmatrix USING btree (rid);


--
-- Name: math_ptptstats_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX math_ptptstats_idx ON public.math_ptptstats USING btree (zid);


--
-- Name: participants_conv_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX participants_conv_idx ON public.participants USING btree (zid);


--
-- Name: participants_conv_uid_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX participants_conv_uid_idx ON public.participants USING btree (uid);


--
-- Name: site_domain_whitelist_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX site_domain_whitelist_idx ON public.users USING btree (site_id);


--
-- Name: suzinvites_owner_zid_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX suzinvites_owner_zid_idx ON public.suzinvites USING btree (owner, zid);


--
-- Name: users_uid_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX users_uid_idx ON public.users USING btree (uid);


--
-- Name: votes_latest_unique_zid_tid_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX votes_latest_unique_zid_tid_idx ON public.votes USING btree (zid, tid);


--
-- Name: votes_zid_pid_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX votes_zid_pid_idx ON public.votes USING btree (zid, pid);


--
-- Name: xid_whitelist_owner_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX xid_whitelist_owner_idx ON public.xid_whitelist USING btree (owner);


--
-- Name: xids_owner_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX xids_owner_idx ON public.xids USING btree (owner);


--
-- Name: zinvites_zid_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX zinvites_zid_idx ON public.zinvites USING btree (zid);


--
-- Name: votes on_vote_insert_update_unique_table; Type: RULE; Schema: public; Owner: postgres
--

CREATE RULE on_vote_insert_update_unique_table AS
    ON INSERT TO public.votes DO  INSERT INTO public.votes_latest_unique (zid, pid, tid, vote, weight_x_32767, modified)
  VALUES (new.zid, new.pid, new.tid, new.vote, new.weight_x_32767, new.created) ON CONFLICT(zid, pid, tid) DO UPDATE SET vote = excluded.vote, modified = excluded.modified;


--
-- Name: participants pid_auto; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER pid_auto BEFORE INSERT ON public.participants FOR EACH ROW EXECUTE FUNCTION public.pid_auto();


--
-- Name: participants pid_auto_unlock; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER pid_auto_unlock AFTER INSERT ON public.participants FOR EACH ROW EXECUTE FUNCTION public.pid_auto_unlock();


--
-- Name: comments tid_auto; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER tid_auto BEFORE INSERT ON public.comments FOR EACH ROW EXECUTE FUNCTION public.tid_auto();


--
-- Name: comments tid_auto_unlock; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER tid_auto_unlock AFTER INSERT ON public.comments FOR EACH ROW EXECUTE FUNCTION public.tid_auto_unlock();


--
-- Name: apikeysndvweifu apikeysndvweifu_uid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.apikeysndvweifu
    ADD CONSTRAINT apikeysndvweifu_uid_fkey FOREIGN KEY (uid) REFERENCES public.users(uid);


--
-- Name: auth_tokens auth_tokens_uid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.auth_tokens
    ADD CONSTRAINT auth_tokens_uid_fkey FOREIGN KEY (uid) REFERENCES public.users(uid);


--
-- Name: comment_translations comment_translations_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.comment_translations
    ADD CONSTRAINT comment_translations_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: comments comments_zid_pid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.comments
    ADD CONSTRAINT comments_zid_pid_fkey FOREIGN KEY (zid, pid) REFERENCES public.participants(zid, pid);


--
-- Name: contexts contexts_creator_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.contexts
    ADD CONSTRAINT contexts_creator_fkey FOREIGN KEY (creator) REFERENCES public.users(uid);


--
-- Name: contributer_agreement_signatures contributer_agreement_signatures_uid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.contributer_agreement_signatures
    ADD CONSTRAINT contributer_agreement_signatures_uid_fkey FOREIGN KEY (uid) REFERENCES public.users(uid);


--
-- Name: conversation_translations conversation_translations_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.conversation_translations
    ADD CONSTRAINT conversation_translations_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: conversations conversations_course_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.conversations
    ADD CONSTRAINT conversations_course_id_fkey FOREIGN KEY (course_id) REFERENCES public.courses(course_id);


--
-- Name: conversations conversations_org_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.conversations
    ADD CONSTRAINT conversations_org_id_fkey FOREIGN KEY (org_id) REFERENCES public.users(uid);


--
-- Name: conversations conversations_owner_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.conversations
    ADD CONSTRAINT conversations_owner_fkey FOREIGN KEY (owner) REFERENCES public.users(uid);


--
-- Name: courses courses_owner_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.courses
    ADD CONSTRAINT courses_owner_fkey FOREIGN KEY (owner) REFERENCES public.users(uid);


--
-- Name: demographic_data demographic_data_uid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.demographic_data
    ADD CONSTRAINT demographic_data_uid_fkey FOREIGN KEY (uid) REFERENCES public.users(uid);


--
-- Name: facebook_friends facebook_friends_friend_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.facebook_friends
    ADD CONSTRAINT facebook_friends_friend_fkey FOREIGN KEY (friend) REFERENCES public.users(uid);


--
-- Name: facebook_friends facebook_friends_uid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.facebook_friends
    ADD CONSTRAINT facebook_friends_uid_fkey FOREIGN KEY (uid) REFERENCES public.users(uid);


--
-- Name: facebook_users facebook_users_uid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.facebook_users
    ADD CONSTRAINT facebook_users_uid_fkey FOREIGN KEY (uid) REFERENCES public.users(uid);


--
-- Name: inviters inviters_inviter_uid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.inviters
    ADD CONSTRAINT inviters_inviter_uid_fkey FOREIGN KEY (inviter_uid) REFERENCES public.users(uid);


--
-- Name: jianiuevyew jianiuevyew_uid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.jianiuevyew
    ADD CONSTRAINT jianiuevyew_uid_fkey FOREIGN KEY (uid) REFERENCES public.users(uid);


--
-- Name: math_bidtopid math_bidtopid_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.math_bidtopid
    ADD CONSTRAINT math_bidtopid_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: math_cache math_cache_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.math_cache
    ADD CONSTRAINT math_cache_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: math_exportstatus math_exportstatus_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.math_exportstatus
    ADD CONSTRAINT math_exportstatus_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: math_main math_main_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.math_main
    ADD CONSTRAINT math_main_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: math_profile math_profile_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.math_profile
    ADD CONSTRAINT math_profile_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: math_ptptstats math_ptptstats_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.math_ptptstats
    ADD CONSTRAINT math_ptptstats_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: math_report_correlationmatrix math_report_correlationmatrix_rid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.math_report_correlationmatrix
    ADD CONSTRAINT math_report_correlationmatrix_rid_fkey FOREIGN KEY (rid) REFERENCES public.reports(rid);


--
-- Name: math_ticks math_ticks_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.math_ticks
    ADD CONSTRAINT math_ticks_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: metrics metrics_uid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.metrics
    ADD CONSTRAINT metrics_uid_fkey FOREIGN KEY (uid) REFERENCES public.users(uid);


--
-- Name: notification_tasks notification_tasks_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.notification_tasks
    ADD CONSTRAINT notification_tasks_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: page_ids page_ids_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.page_ids
    ADD CONSTRAINT page_ids_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: participant_locations participant_locations_uid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participant_locations
    ADD CONSTRAINT participant_locations_uid_fkey FOREIGN KEY (uid) REFERENCES public.users(uid);


--
-- Name: participant_locations participant_locations_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participant_locations
    ADD CONSTRAINT participant_locations_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: participant_metadata_answers participant_metadata_answers_pmqid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participant_metadata_answers
    ADD CONSTRAINT participant_metadata_answers_pmqid_fkey FOREIGN KEY (pmqid) REFERENCES public.participant_metadata_questions(pmqid);


--
-- Name: participant_metadata_answers participant_metadata_answers_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participant_metadata_answers
    ADD CONSTRAINT participant_metadata_answers_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: participant_metadata_choices participant_metadata_choices_pmaid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participant_metadata_choices
    ADD CONSTRAINT participant_metadata_choices_pmaid_fkey FOREIGN KEY (pmaid) REFERENCES public.participant_metadata_answers(pmaid);


--
-- Name: participant_metadata_choices participant_metadata_choices_pmqid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participant_metadata_choices
    ADD CONSTRAINT participant_metadata_choices_pmqid_fkey FOREIGN KEY (pmqid) REFERENCES public.participant_metadata_questions(pmqid);


--
-- Name: participant_metadata_choices participant_metadata_choices_zid_pid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participant_metadata_choices
    ADD CONSTRAINT participant_metadata_choices_zid_pid_fkey FOREIGN KEY (zid, pid) REFERENCES public.participants(zid, pid);


--
-- Name: participant_metadata_questions participant_metadata_questions_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participant_metadata_questions
    ADD CONSTRAINT participant_metadata_questions_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: participants_extended participants_extended_uid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participants_extended
    ADD CONSTRAINT participants_extended_uid_fkey FOREIGN KEY (uid) REFERENCES public.users(uid);


--
-- Name: participants_extended participants_extended_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participants_extended
    ADD CONSTRAINT participants_extended_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: participants participants_uid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participants
    ADD CONSTRAINT participants_uid_fkey FOREIGN KEY (uid) REFERENCES public.users(uid);


--
-- Name: participants participants_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.participants
    ADD CONSTRAINT participants_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: pwreset_tokens password_reset_tokens_uid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.pwreset_tokens
    ADD CONSTRAINT password_reset_tokens_uid_fkey FOREIGN KEY (uid) REFERENCES public.users(uid);


--
-- Name: report_comment_selections report_comment_selections_rid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.report_comment_selections
    ADD CONSTRAINT report_comment_selections_rid_fkey FOREIGN KEY (rid) REFERENCES public.reports(rid);


--
-- Name: report_comment_selections report_comment_selections_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.report_comment_selections
    ADD CONSTRAINT report_comment_selections_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: reports reports_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.reports
    ADD CONSTRAINT reports_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: social_settings social_settings_uid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.social_settings
    ADD CONSTRAINT social_settings_uid_fkey FOREIGN KEY (uid) REFERENCES public.users(uid);


--
-- Name: suzinvites suzinvites_owner_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.suzinvites
    ADD CONSTRAINT suzinvites_owner_fkey FOREIGN KEY (owner) REFERENCES public.users(uid);


--
-- Name: suzinvites suzinvites_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.suzinvites
    ADD CONSTRAINT suzinvites_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: twitter_users twitter_users_uid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.twitter_users
    ADD CONSTRAINT twitter_users_uid_fkey FOREIGN KEY (uid) REFERENCES public.users(uid);


--
-- Name: upvotes upvotes_uid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.upvotes
    ADD CONSTRAINT upvotes_uid_fkey FOREIGN KEY (uid) REFERENCES public.users(uid);


--
-- Name: upvotes upvotes_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.upvotes
    ADD CONSTRAINT upvotes_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- Name: xid_whitelist xid_whitelist_owner_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.xid_whitelist
    ADD CONSTRAINT xid_whitelist_owner_fkey FOREIGN KEY (owner) REFERENCES public.users(uid);


--
-- Name: xids xids_owner_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.xids
    ADD CONSTRAINT xids_owner_fkey FOREIGN KEY (owner) REFERENCES public.users(uid);


--
-- Name: xids xids_uid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.xids
    ADD CONSTRAINT xids_uid_fkey FOREIGN KEY (uid) REFERENCES public.users(uid);


--
-- Name: zinvites zinvites_zid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.zinvites
    ADD CONSTRAINT zinvites_zid_fkey FOREIGN KEY (zid) REFERENCES public.conversations(zid);


--
-- PostgreSQL database dump complete
--

