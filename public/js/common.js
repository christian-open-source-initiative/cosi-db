// Common class definitions.
class DataRenderer {
    setData(d) {
        this.data = d
        return this;
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

// Basic Renderer that Disables Forms
class DisabledFormEntry extends DataRenderer {

}
