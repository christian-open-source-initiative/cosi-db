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

function escapeHtml(unsafe)
{
    return unsafe
         .replace(/&/g, "&amp;")
         .replace(/</g, "&lt;")
         .replace(/>/g, "&gt;")
         .replace(/"/g, "&quot;")
         .replace(/'/g, "&#039;");
 }

// Mini board consists of the render itself
// as well as the state bar at the top.
class MiniBoard {
    constructor(render, searchDarkener) {
        this.isVisible = false;
        this.states = [];
        this.curForm = null;
        // Function that refreshes the table.
        this.updateTable = null;

        this.searchDarkener = searchDarkener;
        this.render = render;

        // Used for clicking outside the element.
        this.searchDarkener.click(() => {
            this.confirmChanges();
        });

    }

    setUpdateTable(updateTable) {
        this.updateTable = updateTable;
    }

    confirmChanges() {
        if (!this.isVisible)  {return false;}
        let hasAllEmpty = true;
        let allSameToOriginal = true;
        let curState = this.states[this.states.length - 1];
        $("#miniboard-form input[type='text'], #miniboard-form textarea").each(function() {
            let dom = $(this);
            hasAllEmpty &= dom.val() == "";

            // Multi part forms will
            let defVal = curState[dom.attr("name")];
            let checkVal = defVal;
            try {
                checkVal = JSON.parse(defVal);
            } catch {}
            // Could be possible for us to store a value that is JSON-like.
            // Minor inconvenience, however.
            if (Array.isArray(checkVal)) {
                allSameToOriginal &= checkVal.includes(dom.val());
            } else {
                allSameToOriginal &= defVal == dom.val();
            }
        });

        if(hasAllEmpty || allSameToOriginal || confirm("You have unsaved changes. Do you wish to discard?")) {
            this.clearStates();
            return true;
        }
        return false;
    }

    addState(action, state) {
        console.assert(ACTIONS.includes(action), `Invalid action given: ${action}`);
        console.assert(state._fieldNames != null, "Invalid state given.");
        state._action = action;
        this.states.push(state);
        this.updateDisplay();

        $(window).bind("beforeunload", () => {
            return "Have you considered saving?";
        });
    }

    popState() {
        if (this.states.length == 0) {
            return;
        }

        this.states.pop();
        this.render.html("");
        this.updateDisplay();
    }

    clearStates() {
        this.render.html("");
        this.displayOff();
        this.states = [];
        this.curForm = null;

        $(window).bind("beforeunload", null);
    }

    updateDisplay() {
        if (this.states.length > 0) {
            this.displayOn();
        }
        else {
            this.displayOff();
        }
    }

    updateStatus(text, isError=false) {
        let status = $("#miniboard-form-status");
        status.removeClass();
        status.hide();
        status.text(text)
        status.fadeIn(200)
        if (isError) {
            status.addClass("error-msg");
        } else {
            status.addClass("success-msg");
        }
    }

    getStateRender(state) {
        // Debug for creating default template.
        let formName = state._stateName.toLowerCase();
        let action = state._action;
        let result = "";
        if (action == "insert") {
            result += `<form id='miniboard-form' action='/insert_${formName}' method='post' novalidate>`;
            result += `<h1 id='miniboard-form-title'>Add New ${state._stateName}</h1>`
        } else {
            result += `<form id='miniboard-form' action='/update_${formName}?oid=${state._oid}' method='post' novalidate>`;
            result += `<h1 id='miniboard-form-title'>Update ${state._stateName}</h1>`
        }
        result += "<div id='miniboard-form-body'>";

        let groupTrack = 0;
        const textAreaThreshold = 256;
        result += `<div id='miniboard-form-group-${formName}-${groupTrack}' class='miniboard-form-group'>`;

        state._fieldNames.forEach((field, idx) => {
            let constraint = state._constraints[field];
            let custom = state._custom[field] || {};
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
            let lengthMeta = constraint.length;
            if (lengthMeta != null) {
                let maxLength = lengthMeta.maximum ? Math.min(lengthMeta.maximum, 50) : 20;
                if (maxLength <= textAreaThreshold) {
                    extraStyle += `width: ${maxLength * 0.75}rem;`;
                }
            }

            // Different inputs for each validator.
            let defStyle = `style="${extraStyle}" class="miniboard-form-input" id="miniboard-form-input-${field}" name="${field}"`;
            let defValue = "";
            if (state[field]) {
                defValue = `value="${escapeHtml(state[field])}"`;
            }

            if (constraint.datetime && constraint.datetime.dateOnly) {
                result += `<input ${defStyle} type='date' placeholder='${field}' ${defValue}/>`;
            } else if (constraint.datetime) {
                result += `<input ${defStyle} type='datetime-local' placeholder='${field}' ${defValue} />`;
            } else if(constraint.length && constraint.length.maximum > textAreaThreshold) {
                result += `<textarea ${defStyle} type='textarea' placeholder='${field}' ${defValue}></textarea>`;
            } else if(custom.options) {
                // Options expansion.
                result += `<select ${defStyle} type='select' placeholder='${field}'>`;
                if (custom.nullable) {
                    result += `<option disabled selected value>--no-option--</option>`
                }
                custom.options.forEach((opt) => {
                    result += `<option value=${opt}>${opt}</option>`;
                });
                result += `</select>`
            } else if (custom.checklist) {
                // Checklist expansion.
                result += `<div class="miniform-form-checkbox">`
                let arr = state[field] ? JSON.parse(state[field]) : [];
                custom.checklist.forEach((opt) => {
                    result += `<div class="miniform-form-checkbox-option">`
                    result += `<label>${opt}</label>`
                    let checkedSetting = arr.includes(opt) ? "checked" : "";
                    result += `<input ${defStyle} value="${opt}" type='checkbox' ${checkedSetting}/>`
                    result += `</div>`
                });
                result += `</div>`
            } else if (custom.vectorize) {
                result += `<div class="miniboard-form-vectorized" name="${field}">`
                let arr = state[field] ? JSON.parse(state[field]): [];
                arr.forEach((val) => {
                    result += `<input class="miniboard-form-input miniboard-form-input-vectorized" name="${field}" value="${val}" type="text"/>`
                })
                result += `<div>`
                result += `<button class="miniboard-add-vectorized">+</button>`
                result += `<button class="miniboard-sub-vectorized">-</button>`
                result += `</div>`
                result += `</div>` // close vectorization
            } else {
                result += `<input ${defStyle} type='text' placeholder='${field}' ${defValue}/>`;
            }
            result += `</div>` // close form entry.
        });

        result += "</div>"; // close form group
        result += "</div>"; // close form body
        result += "<div id='miniboard-form-status'></div>"
        if (action == "insert") {
            result += "<input type='submit' value='Add'/>"
        } else {
            result += "<input type='submit' value='Update'/>"
        }
        result += "</form>"; // close form
        return result;
    }

    updateAllStatusForInput(errors) {
        // Used for finalizing the form.
        $.each(errors, (name, errVal) => {
            this.updateStatusForInput(errVal, name, true);
        });
    }

    updateStatusForInput(error, inputName, finalize=false) {
        // finalize allows for check of blank input for form.
        // which we normally don't check by default.
        //
        // We don't want to check the checkbox as each checkbox is its own input and can grow unwieldly.
        let inputDom = $(`input[name="${inputName}"][type!="checkbox"], textarea[name="${inputName}"]`);
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
      let curState = this.states[this.states.length - 1];
      let errors = validate(this.curForm, curState._constraints);
      console.log(errors);
      // then we update the form to reflect the results
      if (!errors) {
        this.updateStatus("Submitting...", false)
        // We want empty forms to denote nullable.
        let serializedForm = this.curForm.find(":input").filter((idx, element) => {
            let dom = $(element);
            let nullable = (curState._custom[dom.attr("name")] || {} ).nullable == true;
            return  dom.val() != "" || !nullable;
        }
        ).serialize();
        console.log(serializedForm);

        $.ajax({
            type: "POST",
            url: this.curForm.attr("action"),
            data: serializedForm,
            success: (response) => {
                if(this.curForm.attr("action").includes("update") && response == 0) {
                    this.updateStatus("No updates received.", true);
                    return;
                }
                this.updateStatus("Successfully added new row!", false)
                this.popState();
            },
            error: (response) => {
                if (response.responseJSON) {
                    this.updateStatus(`Error adding data: ${response.responseJSON["err"]}`, true)
                } else {
                    this.updateStatus(`Error adding data: ${response.statusText}`, true)
                }
            }
        });
      } else {
        this.updateStatus("Invalid input detected.", true)
        this.updateAllStatusForInput(errors);
      }
    }

    displayOn() {
        const fadein = 200;

        let curState = this.states[this.states.length - 1];
        this.searchDarkener.fadeIn(fadein);
        this.render.append(this.getStateRender(curState));
        this.render.fadeIn(fadein);
        this.isVisible = true;

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
            input.on("change", () => {
                // Sometimes valid returns undefined fully. We need to have valid state for subsequent calls too.
                let errors = validate(that.curForm, that.states[that.states.length-1]._constraints) || {};
                that.updateStatusForInput(errors[name], name);
            });
        });

        // Add plus and minus button listeners for vectorizers
        $(".miniboard-add-vectorized").each(function() {
            let dom = $(this);
            dom.on("click", (ev) => {
                ev.preventDefault();
                let name = dom.parent().parent().attr("name");
                let last = dom.parent().parent().find("div").last();
                // Empty check.
                last.before(`<input class="miniboard-form-input miniboard-form-input-vectorized" name="${name}" type="text"/>`)
                // $(".miniboard-sub-vectorized").show();
            })
        });

        $(".miniboard-sub-vectorized").each(function() {
            let dom = $(this);
            dom.on("click", (ev) => {
                ev.preventDefault();
                let lastEntry = dom.parent().parent().find(".miniboard-form-input-vectorized").last();
                if (lastEntry) {
                    lastEntry.remove();
                }
            })
        });
    }

    displayOff() {
        this.searchDarkener.fadeOut();
        this.render.hide(300);
        this.isVisible = false;
        this.updateTable();``
    }
}
