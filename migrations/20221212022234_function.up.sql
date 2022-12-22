CREATE OR REPLACE FUNCTION euro.query(
    bid INTEGER,  --bookmaker id
    mid INTEGER,  --match id
    lid INTEGER,  --league id
    tid INTEGER,  --team id
    game_year VARCHAR,
    game_round VARCHAR,
    is_desc bool DEFAULT false,
    cursor bigint default null,
    page_size bigint default 10
) RETURNS TABLE (LIKE euro.matches) AS $$
DECLARE
    _sql text;
    BEGIN
        -- if cursor is null, set it to 0 if is_desc is false, or to max int if is_desc is true
        IF cursor IS NULL or cursor < 0 THEN
            IF is_desc THEN
                cursor := 2147483647;
            ELSE
                cursor := 0;
            END IF;
        END IF;
        -- if page_size is not between 10 and 100, set it to 10
        IF page_size < 10 OR page_size > 100 THEN
            page_size := 10;
        END IF;
        -- format the qurey based on parameters
        _sql := format(
            'select matches.* from euro.matches matches, euro.odds odds
            where matches.id == odds.id and %s and %s and %s and %s and %s order by id %s limit %L::integer',
            CASE
                WHEN is_desc THEN 'odds.id < ' || cursor
                ELSE 'odds.id > ' || cursor
            END,
            CASE
                WHEN lid IS NULL AND tid IS NULL  THEN 'TRUE'
                WHEN lid IS NULL THEN ('matches.home_team_id = ' || quote_literal(tid)
                                        or 'matches.away_team_id = ' || quote_literal(tid))
                WHEN tid IS NULL THEN 'matches.league_id = ' || quote_literal(lid)
                ELSE 'matches.league_id  =' || quote_literal(lid) || ' AND (matches.home_team_id = ' || quote_literal(tid)
                                        or 'matches.away_team_id = ' || quote_literal(tid) || ')'
            END,
            CASE
                WHEN bid IS NULL AND mid IS NULL  THEN 'TRUE'
                WHEN bid IS NULL THEN 'odds.match_id = ' || quote_literal(mid)
                WHEN mid IS NULL THEN 'odds.bookmaker_id = ' || quote_literal(bid)
                ELSE 'bookmaker_id =' || quote_literal(bid) || ' AND match_id = ' || quote_literal(mid)
            END,
            CASE
                WHEN game_year IS NULL THEN 'TRUE'
                ELSE 'game_year =' || quote_literal(game_year)
            END,
            CASE
                WHEN game_round IS NULL THEN 'TRUE'
                ELSE 'game_round =' || quote_literal(game_round)
            END,
            CASE
                WHEN is_desc THEN 'DESC'
                ELSE 'ASC'
            END,
            page_size
        );

        -- log the sql
        RAISE NOTICE '%', _sql;

        -- execute the query
        RETURN QUERY EXECUTE _sql;

    END;
$$ LANGUAGE plpgsql;
