// In charge of rendering the expanded data view.
class PersonExpanded extends DataRenderer {
    div(cls, inner) {
        if (cls == null)  {cls = ""; }
        else { cls = "-" + cls; }
        return `
            <div class="${this.defCSSPrefix}-person${cls}">
                ${inner}
            </div>
        `;
    }

    render(cb, fcb) {
        let nicks = "";
        let result = this.div(null,
            this.div("profile", "") +
            this.div("person-name", this.data["first_name"] + " " + this.data["last_name"]) +
            this.div("sex", this.data["sex"]) +
            this.div("nicknames", this.data["nicks"]) +
            this.div("phone", "") +
            this.div("notes", this.data["notes"]) +
            this.div("emergency", this.data["emergency"])
        );
        cb(result);
    }
}
