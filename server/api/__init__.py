from flask import Flask, render_template
from flask_cors import CORS
from flask_sqlalchemy import SQLAlchemy
from config import env

app = Flask(__name__)
CORS(app)

app.config["SQLALCHEMY_DATABASE_URI"] = env["DATABASE_URI"]
app.config["SQLALCHEMY_TRACK_MODIFICATIONS"] = False
