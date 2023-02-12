// Common class definitions.
class DataRenderer {
    setData(d) {
        this.data = d
        return this;
    }

    render() {
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
