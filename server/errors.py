class APIError(Exception):
    """Custom 400-405 Error"""
    def __init__(
        self,
        message: str,
        status: int = 400,
    ):
        self.message = str(status) + " " + message
