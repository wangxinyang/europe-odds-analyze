CREATE TABLE euro.bookmakers (
    id SERIAL NOT NULL,
    name VARCHAR(30) NOT NULL,
    url VARCHAR(100) DEFAULT NULL,
    note VARCHAR(50) DEFAULT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT bookmakers_pkey PRIMARY KEY (id),
    CONSTRAINT bookmakers_name_key UNIQUE (name)
);

CREATE TABLE euro.leagues (
    id SERIAL NOT NULL,
    name VARCHAR(30) NOT NULL,
    note VARCHAR(50) DEFAULT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT leagues_pkey PRIMARY KEY (id),
    CONSTRAINT leagues_name_key UNIQUE (name)
);

CREATE TABLE euro.teams (
    id SERIAL NOT NULL,
    league_id SERIAL NOT NULL,
    name VARCHAR(30) NOT NULL,
    note VARCHAR(50) DEFAULT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT teams_pkey PRIMARY KEY (id),
    CONSTRAINT teams_name_key UNIQUE (name)
);

CREATE INDEX teams_league_id_idx ON euro.teams (league_id);

CREATE TABLE euro.matches (
    id SERIAL NOT NULL,
    league_id SERIAL NOT NULL,
    league_name  VARCHAR(30) NOT NULL,
    home_team_id SERIAL NOT NULL,
    home_team VARCHAR(15) NOT NULL,
    away_team_id SERIAL NOT NULL,
    away_team VARCHAR(15) NOT NULL,
    game_time TIMESTAMP NOT NULL,
    game_year VARCHAR(5) DEFAULT NULL,
    game_round VARCHAR(5) DEFAULT NULL,
    game_result VARCHAR(5) DEFAULT NULL,
    history_note VARCHAR(255) DEFAULT NULL,
    note VARCHAR(100) DEFAULT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT matches_pkey PRIMARY KEY (id)
);

CREATE INDEX matches_league_id_idx ON euro.matches (league_id);

CREATE TABLE euro.odds (
    id SERIAL NOT NULL,
    match_id SERIAL NOT NULL,
    bookmaker_id SERIAL NOT NULL,
    bookmaker_name VARCHAR(30) NOT NULL,
    home_win_start NUMERIC(5,2) DEFAULT 0.0,
    draw_start NUMERIC(5,2) DEFAULT 0.0,
    away_win_start NUMERIC(5,2) DEFAULT 0.0,
    home_win_end NUMERIC(5,2) DEFAULT 0.0,
    draw_end NUMERIC(5,2) DEFAULT 0.0,
    away_win_end NUMERIC(5,2) DEFAULT 0.0,
    note VARCHAR(50) DEFAULT NULL,

    CONSTRAINT odds_pkey PRIMARY KEY (id),
    CONSTRAINT odds_match_id_fkey FOREIGN KEY (match_id) REFERENCES euro.matches(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE INDEX odds_match_id_idx ON euro.odds (match_id);
CREATE INDEX odds_bookmaker_id_idx ON euro.odds (bookmaker_id);
