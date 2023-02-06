//** Stores encoding of forms. **/

function FormStruct(stateName, constraints, custom={}, groups=null, prefixHtml="") {
    var fields = Object.keys(constraints);
    var count = fields.length;
    function constructor(params = {}) {
        $.each(params, (key, val) => {
            this[key] = val;
        });
        // internal field names tracker.
        this._fieldNames = fields;
        this._stateName = stateName;

        // Internal tracker that should be set.
        this._action = null;
        this._constraints = constraints;
        this._groups = groups != null ? groups : [count];
        // Stores custom render properties.
        this._custom = custom;

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

const SEX_OPTIONS = [
    "Undefined",
    "Male",
    "Female"
];

const DAY_OPTIONS = [
    "M", "Tu", "W", "Th", "F", "Sa", "Su"
];

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
        "dob": {
            presence: false,
            datetime: {
                dateOnly: true
            }
        },
        "sex": {
            presence: false
        },
        "home_phone": {
            presence: false,
            numericality: true,
            length: {
                maximum: 25
            }
        },
        "work_phone": {
            presence: false,
            numericality: true,
            length: {
                maximum: 25
            }
        },
        "mobile_phone": {
            presence: false,
            numericality: true,
            length: {
                maximum: 25
            }
        },
        "emergency_contact": {
            presence: false,
            numericality: true,
            length: {
                maximum: 25
            }
        },
        "notes": {
            presence: false,
            length: {
                maximum: 2048 * 4
            }
        },
    },
    {
     "sex": {options: SEX_OPTIONS},
     "dob": {nullable: true},
     "nicks": {nullable: true, vectorize: true},
     "home_phone": {nullable: true},
     "work_phone": {nullable: true},
     "mobile_phone": {nullable: true}
    },
    [4, 6, 10, 11]
);

let GroupState = FormStruct(
    "Group",
    {
        "group_name": {
            presence: true
        },
        "group_desc": {
            presence: true
        }
    }
);

let AddressState = FormStruct(
    "Address",
    {
        "line_one": {
            presence: true
        },
        "line_two": {
            presence: false
        },
        "line_three": {
            presence: false
        },
        "city": {
            presence: true
        },
        "region": {
            presence: true,
        },
        "postal_code": {
            presence: true,
        },
        "county": {
            presence: false
        },
        "country": {
            presence: true
        },
    },
    {},
    [3,9]
);

let EventState = FormStruct(
    "Event",
    {
        "name": {
            presence: true,
            length: {
                maximum: 64
            }
        },
        "meeting_days": {
            presence: true
        },
        "start_datetime": {
            datetime: true,
            presence: true
        },
        "end_datetime": {
            datetime: true
        },
        "freq": {
            numericality: true
        },
        "reoccuring": {
        }
    },
    {
        "meeting_days": {
            checklist: DAY_OPTIONS
        },
        "end_datetime": {
            nullable: true
        },
        "reoccuring": {
            nullable: true,
            options: ["Days", "Weeks", "Months"]
        }
    },
    [1, 4, 6]
);
