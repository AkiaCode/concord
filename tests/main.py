import concord


client = concord.Client("TOKEN")


@client.on('Ready')
def on_ready():
    print('ready!')

@client.on('MessageCreate')
def on_message(raw_message):
    message = concord.Message(raw_message)
    print(message.author.username + ": " +  message.content)

    if message.content == '!ping':
        message = client.send_message(message.channel_id, "pong")
        print(message)

client.run()