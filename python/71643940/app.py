from flask import Flask, render_template, request

app = Flask(__name__)

data = {
    "departments": [{
        "id": 1,
        "name": "Dept 1",
    }, {
        "id": 2,
        "name": "Dept 2",
    }, {
        "id": 3,
        "name": "Dept 3",
    }],
    "cardgroups": [{
        "id": 4,
        "name": "Card 1",
    }, {
        "id": 5,
        "name": "Card 2",
    }, {
        "id": 6,
        "name": "Card 3",
    }],
    "doorgroups": [{
        "id": 7,
        "name": "Door 1",
    }, {
        "id": 8,
        "name": "Door 2",
    }, {
        "id": 9,
        "name": "Door 3",
    }],
}

@app.route('/v2', methods=["GET", "POST"])
def v2():
    print(request.form)
    return render_template('v2.html', data=data)

@app.route('/v1', methods=["GET", "POST"])
def v1():
    print(request.form)
    return render_template('v1.html', data=data)

app.run(debug=True)
