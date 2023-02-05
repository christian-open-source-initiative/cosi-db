/**
* Logic for the secondary board that appears.
*/
const ACTION_UPDATE = "update";
const ACTION_INSERT = "insert";
const ACTIONS = [
    ACTION_INSERT,
    ACTION_UPDATE
];

validate.extend(validate.validators.datetime, {
  // The value is guaranteed not to be null or undefined but otherwise it
  // could be anything.
  parse: function(value, options) {
    return +moment.utc(value);
  },
  // Input is a unix timestamp
  format: function(value, options) {
    var format = options.dateOnly ? "YYYY-MM-DD" : "YYYY-MM-DD hh:mm:ss";
    return moment.utc(value).format(format);
  }
});

function FormStruct(stateName, constraints, groups=null, prefixHtml="") {
    var fields = Object.keys(constraints);
    var count = fields.length;
    function constructor() {
        for (var i = 0; i < count; ++i) {
            if (i < arguments.length) {
                this[fields[i]] = arguments[i];
            }
            else {
                // Set default argument value instead.
                this[fields[i]] = ""
            }
        }
        // internal field names tracker.
        this._fieldNames = fields;
        this._stateName = stateName;

        // Internal tracker that should be set.
        this._action = null;
        this._constraints = constraints;
        this._groups = groups != null ? groups : [count];

        // For use on special things like profile pics, etc.
        this.prefixHtml = prefixHtml;

        this.equals = (other) => {
            for (var i = 0; i < count; ++i) {
                if (this[fields[i]] != other[fields[i]]) {
                    return false;
                }
            }
            return true;
        };
    }
    return constructor;
}

// State for tracking people.
let PersonState = FormStruct(
    "Person",
    {
        "first_name": {
            presence: {allowEmpty: false},
            length: {
                maximum: 30
            }
        },
        "middle_name": {
            presence: false,
            length: {
                maximum: 30
            }
        },
        "last_name": {
            presence: {allowEmpty: false},
            length: {
                maximum: 30
            }
        },
        "nicks": {
            presence: false,
            length: {
                maximum: 30
            }
        },
        "sex": {
            presence: false
        },
        "dob": {
            presence: false
        },
        "home_phone": {
            presence: false,
            numericality: true,
            length: {
                maximum: 30
            }
        },
        "work_phone": {
            presence: false,
            numericality: true,
            length: {
                maximum: 30
            }
        },
        "mobile_phone": {
            presence: false,
            numericality: true,
            length: {
                maximum: 30
            }
        },
        "emergency": {
            presence: false
        },
        "notes": {
            presence: false
        },
    },
    [4, 6, 9, 11]
);

// Mini board consists of the render itself
// as well as the state bar at the top.
class MiniBoard {
    constructor(render, searchDarkener) {
        this.isVisible = false;
        this.states = [];
        this.curForm = null;

        this.searchDarkener = searchDarkener;
        this.render = render;
    }

    addState(action, state) {
        console.assert(ACTIONS.includes(action), `Invalid action given: ${action}`);
        console.assert(state._fieldNames != null, "Invalid state given.");
        state._action = action;
        this.states.push(state);
        this.updateDisplay();
    }

    updateDisplay() {
        if (this.states.length > 0) {
            this.displayOn();
        }
        else {
            this.displayOff();
        }
    }

    _getStateRender(state) {
        let xmlhttp = new XMLHttpRequest();
        xmlhttp.open("GET", "public/forms/person_form.html", false);
        xmlhttp.send();
        if (xmlhttp.status == 200) {
            return xmlhttp.responseText;
        }
        return "An error occurred, please try again."
    }

    getStateRender(state) {
        // Debug for creating default template.
        let result = "<form id='miniboard-form' action='/insert_person' method='post' novalidate>";
        result += `<h1 id='miniboard-form-title'>Add New ${state._stateName}</h1>`
        result += "<div id='miniboard-form-body'>";

        let groupTrack = 0;
        let formName = state._stateName.toLowerCase();
        result += `<div id='miniboard-form-group-${formName}-${groupTrack}' class='miniboard-form-group'>`;
        state._fieldNames.forEach((field, idx) => {
            let g = state._groups[groupTrack];
            if (idx >= g) {
                groupTrack += 1;
                result += `</div>`; // close form group
                result += `<div id='miniboard-form-group-${formName}-${groupTrack}' class='miniboard-form-group'>`;
            }

            // Used for unique css labeled by state name and then field.
            result += `<div class='miniboard-form-entry' id='miniboard-form-entry-${formName}-${field}'>`
            // Required check
            result += `<h2 class='miniboard-form-entry-name'>${field}`;
            if (state._constraints[field].presence) {
                result += "<div class='miniboard-form-required-asterisk'> *</div>";
            }
            result += "</h2>"

            let extraStyle = "";
            let lengthMeta = state._constraints[field].length;
            if (lengthMeta != null) {
                let maxLength = lengthMeta.maximum ? lengthMeta.maximum : 20;
                extraStyle += `width: ${maxLength}rem;`;
            }
            result += `<input style='${extraStyle}' class='miniboard-form-input' id='miniboard-form-input-${field}' type='text' placeholder='${field}' name='${field}' />`;
            result += `</div>` // close form entry.
        });

        result += "</div>"; // close form group
        result += "</div>"; // close form body
        result += "<input type='submit'>"
        result += "</form>"; // close form
        return result;
    }

    updateAllStatusForInput(errors) {
        $.each(errors, (name, errVal) => {
            this.updateStatusForInput(errVal, name, true);
        });
    }

    updateStatusForInput(error, inputName, finalize=false) {
        // finalize allows for check of blank input for form.
        // which we normally don't check by default.
        //
        let inputDom = $(`input[name="${inputName}"]`);
        let msgDom = $(`.error-msg[name="${inputName}"]`);
        msgDom.remove();
        inputDom.removeClass("has-error");
        inputDom.removeClass("has-success");
        if (!finalize && inputDom.val() == "") {
            // Don't color if no input.
            return;
        }
        else if (!error) {
            inputDom.addClass("has-success");
            return;
        }
        inputDom.before(`<div class='error-msg' name='${inputName}'>${error[0]}</div>`);
        inputDom.addClass("has-error");
    }

    _handleFormSubmit() {
      // validate the form against the constraints
      let errors = validate(this.curForm, this.states[this.states.length-1]._constraints) || {};
      console.log(errors);
      // then we update the form to reflect the results
      this.updateAllStatusForInput(errors);
      if (errors) {

      }
    }

    displayOn() {
        let curState = this.states[this.states.length - 1];
        this.searchDarkener.show();
        this.render.append(this.getStateRender(curState));
        this.render.show();

        this.curForm = $("#miniboard-form")
        this.curForm.submit((ev) => {
            ev.preventDefault();
            this._handleFormSubmit();
        });

        // Hook up auto listeners
        let allInputs = $("input, textarea, select");
        let that = this;
        allInputs.each(function() {
            let input = $(this);
            let name = input.attr("name");
            input.change(() => {
                // Sometimes valid returns undefined fully. We need to have valid state for subsequent calls too.
                let errors = validate(that.curForm, that.states[that.states.length-1]._constraints) || {};
                that.updateStatusForInput(errors[name], name);
            });
        });
    }

    displayOff() {
        this.render.hide();
        this.searchDarkener.hide();
    }
}
