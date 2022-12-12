CREATE OR REPLACE FUNCTION euro.query(
    bid SERIAL, 
    mid SERIAL, 
    year INTEGER, 
    is_desc bool DEFAULT false,
    cursor bigint default null,
    page_size bigint default 10
) RETURNS TABLE (LIKE euro.odds) AS $$
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
            'select * from euro.odds where %s and %s order by id %s limit %L::integer',
            CASE
                WHEN is_desc THEN 'id < ' || cursor
                ELSE 'id > ' || cursor
            END,
            CASE
                WHEN bid IS NULL AND mid IS NULL  THEN 'TRUE'
                WHEN bid IS NULL THEN 'match_id = ' || quote_literal(mid)
                WHEN mid IS NULL THEN 'bookmaker_id = ' || quote_literal(bid)
                ELSE 'bookmaker_id =' || quote_literal(bid) || ' AND match_id = ' || quote_literal(mid)
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