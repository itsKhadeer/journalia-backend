CREATE TYPE user_role AS ENUM ('user', 'writer', 'moderator', 'admin');
CREATE TABLE public.users (
  user_id serial NOT NULL,
  user_name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL UNIQUE,
  phone VARCHAR(255) NOT NULL UNIQUE,
  access_token VARCHAR(255),
  role user_role NOT NULL DEFAULT 'user',
  CONSTRAINT user_id_pk PRIMARY KEY (user_id)
) WITH (OIDS = FALSE);
CREATE TABLE public.topics (
  topic_id serial NOT NULL,
  topic_name VARCHAR(255) NOT NULL,
  CONSTRAINT topic_id_pk PRIMARY KEY (topic_id)
) WITH (OIDS = FALSE);
CREATE TABLE public.articles (
  article_id serial NOT NULL UNIQUE,
  topic_id INTEGER NOT NULL,
  writer_id INTEGER NOT NULL,
  title VARCHAR(255) NOT NULL,
  image_url VARCHAR(255),
  content TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  CONSTRAINT article_id_pk PRIMARY KEY (article_id),
  CONSTRAINT article_topic_id_fk FOREIGN KEY (topic_id) REFERENCES public.topics(topic_id),
  CONSTRAINT article_writer_id_fk FOREIGN KEY (writer_id) REFERENCES public.users(user_id)
) WITH (OIDS = FALSE);
CREATE TABLE public.comments (
  comment_id INTEGER NOT NULL,
  article_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  content TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  CONSTRAINT comment_id_pk PRIMARY KEY (comment_id),
  CONSTRAINT comment_article_id_fk FOREIGN KEY (article_id) REFERENCES public.articles(article_id),
  CONSTRAINT comment_user_id_fk FOREIGN KEY (user_id) REFERENCES public.users(user_id)
) WITH (OIDS = FALSE);
CREATE TYPE vote_value AS ENUM ('up', 'down');
CREATE TABLE public.votes (
  vote_id serial NOT NULL,
  article_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  vote_type vote_value NOT NULL,
  CONSTRAINT vote_id_pk PRIMARY KEY (vote_id),
  CONSTRAINT vote_article_id_fk FOREIGN KEY (article_id) REFERENCES public.articles(article_id),
  CONSTRAINT vote_user_id_fk FOREIGN KEY (user_id) REFERENCES public.users(user_id)
) WITH (OIDS = FALSE);