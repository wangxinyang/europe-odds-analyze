CREATE OR REPLACE FUNCTION euro.query(
    bid INTEGER,  --bookmaker id
    lid INTEGER,  --league id
    tid INTEGER,  --team id
    game_year VARCHAR,
    game_round VARCHAR,
    is_desc bool DEFAULT false,
    page integer default null,
    page_size integer default 10
) RETURNS TABLE (LIKE euro.matches) AS $$
DECLARE
    _sql text;
    BEGIN
        -- if cursor is null, set it to 0 if is_desc is false, or to max int if is_desc is true
        IF page_size > 100 THEN
            page_size := 10;
        END IF;
        IF page < 1 THEN
            page := 1;
        END IF;
        -- format the qurey based on parameters
        _sql := format(
            'select * from euro.matches where %s and %s and %s and %s
            order by id %s limit %L::integer offset %s',
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
            page_size,
            (page - 1) * page_size
        );

        -- log the sql
        RAISE NOTICE '%', _sql;

        -- execute the query
        RETURN QUERY EXECUTE _sql;

    END;
$$ LANGUAGE plpgsql;
