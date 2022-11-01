from database import db
from api.models.user import User
from api import app
with app.app_context():
    db.create_all()
