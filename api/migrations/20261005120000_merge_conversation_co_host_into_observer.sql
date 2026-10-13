DELETE FROM conversation_user_permissions AS cohost
USING conversation_user_permissions AS observer
WHERE cohost.role_name = 'conversation_co_host'
  AND observer.role_name = 'observer'
  AND cohost.user_id = observer.user_id
  AND cohost.resource_id = observer.resource_id;

UPDATE conversation_user_permissions
SET role_name = 'observer'
WHERE role_name = 'conversation_co_host';

DELETE FROM conversation_group_permissions AS cohost
USING conversation_group_permissions AS observer
WHERE cohost.role_name = 'conversation_co_host'
  AND observer.role_name = 'observer'
  AND cohost.group_id = observer.group_id
  AND cohost.resource_id = observer.resource_id;

UPDATE conversation_group_permissions
SET role_name = 'observer'
WHERE role_name = 'conversation_co_host';