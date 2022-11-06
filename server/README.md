To code:
`source bin/activate`
`pip install -r requirements.txt `
`flask run --host=0.0.0.0`

If pylint throws errors about scoped_session, you can fix them by running:
`sudo -H pip install pylint_flask`
`sudo -H pip install pylint_flask_sqlalchemy`

Debugging:
pip3:
`python3 -m ensurepip`

use ./bin before commands