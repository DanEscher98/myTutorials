# Third-party imports
import openai
from fastapi import FastAPI, Form, Depends, Request
from decouple import config
from sqlalchemy.exc import SQLAlchemyError
from sqlalchemy.orm import Session
from typing import Iterable

# Internal imports
# from wabotgpt.models import Conversation  # , SessionLocal
from wabotgpt.utils import send_message, logger


app = FastAPI()
# Set up the OpenAI API client
openai.api_key = config("OPENAI_KEY")


# Dependency
# def get_db() -> Iterable[Session]:
#     with SessionLocal() as db:
#         yield db


@app.get("/")
async def index():
    return {"msg": "working"}


@app.post("/message")
async def reply(request: Request, body: str = Form()):
    # , db: Session = Depends(get_db)):
    # Extract the phone number from the incoming webhook request
    form_data = await request.form()
    whatsapp_number = str(form_data['From']).split("whatsapp:")[-1]
    print(f"Sending the ChatGPT response to this number: {whatsapp_number}")

    # Call the OpenAI API to generate text with ChatGPT
    messages = [{"role": "user", "content": body}]
    messages.append({"role": "system",
                     "content": "You're a dance schools system"})
    response = openai.ChatCompletion.create(
        model="gpt-3.5-turbo",
        messages=messages,
        max_tokens=200,
        n=1,
        stop=None,
        temperature=0.5
    )

    # The generated text
    chatgpt_response = response.choices[0].message.content

    logger.info(
            f"Conversation happening: {whatsapp_number} -> {chatgpt_response}")
    print(f"{whatsapp_number} -> {chatgpt_response}")
    # Store the conversation in the database
    # try:
    #     conversation = Conversation(
    #         sender=whatsapp_number,
    #         message=body,
    #         response=chatgpt_response
    #         )
    #     # db.add(conversation)
    #     # db.commit()
    #     logger.info(f"Conversation #{conversation.id} stored in database")
    # except SQLAlchemyError as e:
    #     # db.rollback()
    #     logger.error(f"Error storing conversation in database: {e}")
    send_message(whatsapp_number, chatgpt_response)
    return ""
