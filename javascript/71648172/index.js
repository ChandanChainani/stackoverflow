const mongoose = require('mongoose');

mongoose.connect('mongodb://localhost:27017/myapp');

const tmpSchema = mongoose.Schema({
  field1: {
    type: String,
    // required: function() {
    //   if (this.field2 == undefined && this.field3 == undefined) {
    //     return true;
    //   }
    //   return false;
    // },
    validate: {
      validator: function(v) {
        // console.log(this.field2, this.field3);
        if (this.field2 == undefined && this.field3 == undefined) {
          return true;
        }
        return false;
      },
      // message: props => `${props.value} is not a valid!`
    },
  },
  field2: {
    type: String,
    // required: function() {
    //   if (this.field1 == undefined && this.field3 == undefined) {
    //     return true;
    //   }
    //   return false;
    // },
    validate: {
      validator: function(v) {
        // console.log(this.field1, this.field3);
        if (this.field1 == undefined && this.field3 == undefined) {
          return true;
        }
        return false;
      },
      // message: props => `${props.value} is not a valid!`
    },
  },
  field3: {
    type: String,
    // required: function() {
    //   console.log(this.field2);
    //   if (this.field2 != undefined && this.field1 != undefined) {
    //     return true;
    //   }
    //   return false;
    // },
    validate: {
      validator: function(v) {
        if (this.field2 == undefined && this.field1 == undefined) {
          return true;
        }
        return false;
      },
      // message: props => `${props.value} is not a valid!`
    },
  },
});

const MyModel = mongoose.model('Test', tmpSchema);
// console.log(tmpSchema);
// console.log(MyModel);
const my_model = new MyModel({
  // field1: '1',
  field2: '2',
  field3: '3',
});
// my_model.field1 = 'one';
// my_model.field2 = 'two';
// my_model.field3 = 'two';

const errors = my_model.validateSync();
console.log(errors);
// console.log(mongoose);
// console.dir(mongoose);

mongoose.disconnect();

console.log("done");
