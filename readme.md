A bot to run The "Commune" server through a system of direct democracy.

Using the bot, any member of The "Commune" server who has obtained the "Voter" role may initiate a vote for the following actions:

## (actions/channel/delete) Delete a channel
Usage: /propose channel delete \<channel\>

## (actions/channel/purge) Purge messages from a channel
Usage: /propose channel create_voice \<name\> \[category\]

## (actions/channel/textcreate) Create a text channel
Usage: /propose channel create_voice \<name\> \[category\]

## (actions/channel/voicecreate) Create a voice channel
Usage: /propose channel create_voice \<name\> \<topic\> \[category\]

## (actions/channel/categorycreate) Create a category
Usage: /propose channel create_category \<name\>

## (actions/message/create) Send a message
Usage: /propose message create \<channel\> \<message\>
### Params
The 'message' parameter is the text to send.
The 'channel' parameter is the channel to send the text in.

## (actions/message/create) Delete a message
Usage: /propose message delete \<channel\> \<message_id\>
### Params
The 'message_id' parameter is the id of the message to delete.
The 'channel' parameter is the channel to send the text in.

## (actions/role/can_view_send) Make a role able to view and send messages in a channel
Usage: /propose role can_view_send \<role\> \<channel\>
### Params
The 'role' parameter is the role to which this should apply.
The 'channel' parameter is the channel to which this should apply.

## (actions/role/cant_send) Make a role able to view but not to send messages in a channel
Usage: /propose role cant_send \<role\> \<channel\>
### Params
The 'role' parameter is the role to which this should apply.
The 'channel' parameter is the channel to which this should apply.

## (actions/role/cant_send) Make a role unable to view and send messages in a channel
Usage: /propose role cant_view \<role\> \<channel\>
### Params
The 'role' parameter is the role to which this should apply.
The 'channel' parameter is the channel to which this should apply.

## (actions/role/create) Create a role
Usage: /propose role create \<r\> \<g\> \<b\> \<name\> \<position\>

## (actions/role/delete) Delete a role
Usage: /propose role delete \<role\>

## (actions/role/hoist) Makes a role appear on the side bar
Usage: /propose role hoist \<role\>

## (actions/role/hoist) Makes a role not appear on the side bar
Usage: /propose role unhoist \<role\>
### Technical Note
The unhoist command uses the actions/role/hoist API.

## (actions/role/set_position) Changes the position of the role
Usage: /propose role set_position \<role\> \<position\>

## (actions/user/role/add) Add a role to someone
Usage: /propose user role_add \<member\> \<role\>

## (actions/user/role/remove) Remove a role from someone
Usage: /propose user role_remove \<member\> \<role\>
