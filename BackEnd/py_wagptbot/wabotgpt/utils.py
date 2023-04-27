# Standard library import
import logging

# Third-party imports
from twilio.rest import Client
from decouple import config


# Find your Account SID and Auth Token at twilio.com/console
# and set the environment variables. See http://twil.io/secure
account_sid = str(config("TWILIO_ACCOUNT_SID"))
auth_token = str(config("TWILIO_AUTH_TOKEN"))
client = Client(account_sid, auth_token)
twilio_number = config("TWILIO_NUMBER")

# Set up logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


def send_message(to_number, body_text):
    """Sending message logic through Twilio Messaging API"""
    try:
        message = client.messages.create(
            from_=f"whatsapp:{twilio_number}",
            body=body_text,
            to=f"whatsapp:{to_number}"
        )
        logger.info(f"Message sent to {to_number}: {message.body}")
    except Exception as e:
        logger.error(f"Error sending message to {to_number}: {e}")
