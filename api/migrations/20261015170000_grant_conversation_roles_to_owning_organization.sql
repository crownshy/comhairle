-- An owning organization's members can read its conversations and its administrators can edit them.
-- Grants are tagged with a fixed reason so they can be revoked without touching explicit co-host grants.
CREATE FUNCTION sync_conversation_owning_organization_grants () RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'UPDATE' THEN
        DELETE FROM conversation_group_permissions
        WHERE resource_id = OLD.id AND grant_reason = 'Owning organization';
    END IF;

    IF NEW.organization_id IS NOT NULL THEN
        INSERT INTO conversation_group_permissions (group_id, resource_id, role_name, grant_reason)
        SELECT user_group_id, NEW.id, 'observer', 'Owning organization'
        FROM organization WHERE id = NEW.organization_id
        UNION ALL
        SELECT admin_group_id, NEW.id, 'content_editor', 'Owning organization'
        FROM organization WHERE id = NEW.organization_id
        ON CONFLICT DO NOTHING;
    END IF;
    RETURN NULL;
END;
$$;

CREATE TRIGGER conversation_owning_organization_grants_on_insert
AFTER INSERT ON conversation FOR EACH ROW
WHEN (NEW.organization_id IS NOT NULL)
EXECUTE FUNCTION sync_conversation_owning_organization_grants ();

CREATE TRIGGER conversation_owning_organization_grants_on_update
AFTER UPDATE OF organization_id ON conversation FOR EACH ROW
WHEN (OLD.organization_id IS DISTINCT FROM NEW.organization_id)
EXECUTE FUNCTION sync_conversation_owning_organization_grants ();

INSERT INTO conversation_group_permissions (group_id, resource_id, role_name, grant_reason)
SELECT organization.user_group_id, conversation.id, 'observer', 'Owning organization'
FROM conversation JOIN organization ON organization.id = conversation.organization_id
UNION ALL
SELECT organization.admin_group_id, conversation.id, 'content_editor', 'Owning organization'
FROM conversation JOIN organization ON organization.id = conversation.organization_id
ON CONFLICT DO NOTHING;
