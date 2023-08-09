from .concord import *
from .message import Message
from typing import Literal
import json

__all__ = ['Client', 'Message']

class Client:
    def __init__(self, token: str) -> 'Client':
        self.client = concord.Client(token)
        self.request = concord.Request(token)
        self.listeners = {}

    def on(self, event_name: Literal['Ready', 'MessageCreate']):
        def wrapper(func):
            if event_name == 'MessageCreate':
                self.listeners[event_name] = func
            return func
        return wrapper

    def send_message(self, channel_id: str, content: str) -> str:
        return self.request.post("/channels/{}/messages".format(channel_id), json.dumps({ 'content': content }))

    def run(self):
        self.client.run(self.listeners)
