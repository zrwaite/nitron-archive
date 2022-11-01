To code:
`source bin/activate`
`./bin/pip install -r requirements.txt `
`./bin/flask run --host=0.0.0.0`

If pylint throws errors about scoped_session, you can fix them by running:
`sudo -H pip install pylint_flask`
`sudo -H pip install pylint_flask_sqlalchemy`

Debugging:
pip3:
`./bin/python3 -m ensurepip`