import uvicorn


def web_server():
    config = uvicorn.Config(
            "wabotgpt.app:app",
            port=8000,
            log_level="info",
            reload=True)

    server = uvicorn.Server(config)
    server.run()


if __name__ == "__main__":
    web_server()
