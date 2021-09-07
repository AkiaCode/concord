from typing import Optional


class Message(object):
    def __init__(self, raw_message: dict):
        self.raw_message = raw_message
        self.id = raw_message.get('id')
        self.channel_id = raw_message.get('channel_id')
        self.author_id = raw_message.get('author_id')
        self.content = raw_message.get('content')
        self.timestamp = raw_message.get('timestamp')
        self.edited_timestamp = raw_message.get('edited_timestamp')
        self.mentions = raw_message.get('mentions')
        self.attachments = raw_message.get('attachments')
        self.guild_id = raw_message.get('guild_id')
        self.author = Author(raw_message.get('author'))



class Author(object):
    def __init__(self, raw_author: Optional[dict]):
        self.raw_author = raw_author
        self.username = raw_author.get('username')
        self.discriminator = raw_author.get('discriminator')
        self.avatar = raw_author.get('avatar')
        self.id = raw_author.get('id')