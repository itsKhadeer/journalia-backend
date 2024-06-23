-- This file should undo anything in `up.sql`

DROP TABLE public.users;
DROP TABLE public.topics;
DROP TABLE public.articles;
DROP TABLE public.comments;
DROP TABLE public.votes;
DROP TYPE vote_value;
DROP TYPE user_role;
