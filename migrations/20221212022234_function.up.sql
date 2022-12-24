CREATE OR REPLACE FUNCTION euro.query(
    bid INTEGER,  --bookmaker id
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
            'select * from euro.matches where %s and %s and %s and %s and %s order by id %s limit %L::integer',
            CASE
                WHEN is_desc THEN 'id < ' || cursor
                ELSE 'id > ' || cursor
            END,
            CASE
                WHEN lid = 0 AND tid = 0  THEN 'TRUE'
                WHEN lid = 0 THEN '(home_team_id = ' || tid  || 'or away_team_id = ' || tid || ')'
                WHEN tid = 0 THEN 'league_id = ' || lid
                ELSE 'league_id  =' || lid || ' AND (home_team_id = ' || tid  || 'or away_team_id = ' || tid || ')'
            END,
            CASE
                WHEN bid = 0 THEN 'TRUE'
                ELSE 'bookmaker_id =' || bid
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
