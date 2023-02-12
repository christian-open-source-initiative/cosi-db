// Common class definitions.
class DataRenderer {
    setData(d) {
        this.data = d
        return this;
    }

    setDefCSSPrefix(c) {
        this.defCSSPrefix = c;
        return this;
    }

    // Helper for generating cssAttr by the form's standard name.
    cssAttr() {
        // Arguments keywords are not Arrays and require conversion.
        let args = [];
        for (let i = 0; i < arguments.length; ++i) {
            args.push(arguments[i]);
        }
        return this.defCSSPrefix + "-" + args.join("-");
    }

    render(cb, fcb) {
        console.assert("Not yet implemented.")
    }
}

// For rendering data associated with a table entries which has column names.
class CustomTableRender extends DataRenderer {
    setColumn(c) {
        this.column = c;
        return this;
    }
}

class CustomFormEntryRender extends DataRenderer {
    setField(f) {
        this.field = f;
        return this;
    }
}

// Basic Renderer that Disables Forms
class DisabledFormEntry extends CustomFormEntryRender {
    render(cb, fcb) {
        // TODO: Use callback architecture instead.
    }
}
