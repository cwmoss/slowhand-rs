


CREATE TABLE IF NOT EXISTS docs (
    body JSON,
    _id TEXT GENERATED ALWAYS AS (json_extract(body, '$._id'))
        VIRTUAL
        NOT NULL
        ,
    _type TEXT GENERATED ALWAYS AS (json_extract(body, '$._type'))
        VIRTUAL
        NOT NULL
        ,
    _createdAt TEXT GENERATED ALWAYS AS (json_extract(body, '$._createdAt'))
        VIRTUAL
        NOT NULL
        ,
    _updatedAt TEXT GENERATED ALWAYS AS (json_extract(body, '$._updatedAt'))
        VIRTUAL
        NOT NULL
       ,
    _rev TEXT GENERATED ALWAYS AS (json_extract(body, '$._rev'))
        VIRTUAL
        NOT NULL
)
----
CREATE INDEX IF NOT EXISTS docs_id on docs(_id)
----
CREATE INDEX IF NOT EXISTS docs_type on docs(_type)
----
CREATE INDEX IF NOT EXISTS docs_created_at on docs(_createdAt)
----
CREATE TRIGGER IF NOT EXISTS docs_update_notify 
   AFTER UPDATE
   ON docs
BEGIN
    SELECT notify_update(new._id, new._type, new.body);
END
----
CREATE TRIGGER IF NOT EXISTS docs_insert_notify 
   AFTER INSERT
   ON docs
BEGIN
    SELECT notify_insert(new._id, new._type, new.body);
END
----
CREATE VIRTUAL TABLE IF NOT EXISTS docs_fts USING fts5( _id, _type, btext)
----
#CREATE TRIGGER IF NOT EXISTS docs_update_fts 
   AFTER UPDATE
   ON docs
BEGIN
    UPDATE docs_fts 
        SET btext=(
            SELECT group_concat(b.key || ': ' ||  b.value, x'0a') as btext from json_tree(new.body) b where b.atom not null
        ) WHERE _id=new._id;
END
----
#CREATE TRIGGER IF NOT EXISTS docs_insert_fts 
   AFTER INSERT
   ON docs
BEGIN
    INSERT INTO docs_fts(_id, btext)
        SELECT new._id, group_concat(b.key || ': ' ||  b.value, x'0a') as btext from json_tree(new.body) b where b.atom not null;
END
----
#CREATE TRIGGER IF NOT EXISTS docs_delete_fts 
   AFTER DELETE
   ON docs
BEGIN
    DELETE FROM docs_fts where _id = old._id;
END
----
CREATE TABLE IF NOT EXISTS revisions (
    revid TEXT NOT NULL,
    action TEXT NOT NULL,
    actionAt DATE DEFAULT CURRENT_TIMESTAMP,
    oldbody JSON,
    _id TEXT GENERATED ALWAYS AS (json_extract(oldbody, '$._id'))
        VIRTUAL
        NOT NULL
        ,
    _type TEXT GENERATED ALWAYS AS (json_extract(oldbody, '$._type'))
        VIRTUAL
        NOT NULL
        ,
    _updatedAt TEXT GENERATED ALWAYS AS (json_extract(oldbody, '$._updatedAt'))
        VIRTUAL
        NOT NULL
    )
----
CREATE INDEX IF NOT EXISTS revisions_id on revisions(_id)
----
CREATE INDEX IF NOT EXISTS revisions_revid on revisions(revid)
----
CREATE INDEX IF NOT EXISTS revisions_at on revisions(_updatedAt)
----
CREATE TRIGGER IF NOT EXISTS docs_update_revisions
   AFTER UPDATE
   ON docs
BEGIN
    INSERT INTO revisions(revid, action, oldbody)
        SELECT old._rev, 'update', old.body;
END
----
#CREATE TRIGGER IF NOT EXISTS docs_insert_revisions 
   AFTER INSERT
   ON docs
BEGIN
    INSERT INTO revisions(revid, action, oldbody)
        SELECT new._rev, 'insert', new.body;
END
----
CREATE TRIGGER IF NOT EXISTS docs_delete_revisions 
   AFTER DELETE
   ON docs
BEGIN
    INSERT INTO revisions(revid, action, oldbody)
        SELECT old._rev, 'delete', old.body;
END