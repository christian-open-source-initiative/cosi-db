const DEF_PROFILE_PICS = [
    "0be1745835c6d63b5eb2adc8b4aff55b.png",
    "0168b394587296f4bf2f7bc51f3716cf.png",
    "2fefc9ec378a54dd238ab8a270b64057.png",
    "5fa3f6672188533594c0037380c0fc4c.png",
    "8bd986efa280974f18d6022131493d18.png",
    "43a6675bec270585aa57e5c587b4667d.png",
    "b4fe2921307730e156c6372d23ad6ce5.png",
    "e9b9b6401acc0135ca7dbe2f4fe5cd7b.png",
    "00ee287ce8fd30733e9c21f2ca229ff4.png",
    "0290401da7f06d542dac81d40e89c2f5.png",
    "4da520506380b5e95d636123b74f066f.png",
    "5fa667521f763cffd05eb6603e962c9a.png",
    "8cfafb8f3d08d0dc2fabc318875d7641.png",
    "632d14865e8321903e9a0f1fb75114b6.png",
    "b6300c79d2338a339b5aad25d8260ff4.png"
];
// In charge of rendering the expanded data view.

class PersonExpanded extends DataRenderer {
    div(cls, inner) {
        return `
            <div class="${this.cssAttr("person", cls)}">
                ${inner}
            </div>
        `;
    }

    render(cb, fcb) {
        // For now, we mock profile pictures until we have more feature adds.
        let randomIndex = Math.floor(Math.random() * DEF_PROFILE_PICS.length);
        let profilePicUrl = `<img src="public/img/churches/${DEF_PROFILE_PICS[randomIndex]}" class="esearch-person-profile-pic"/>`;
        let result = this.div(null,
            this.div("general-info",
                this.div("profile", profilePicUrl) +
                this.div("name", this.data["first_name"] + " " + this.data["last_name"])
            ) +
            this.div("main-section",
                this.div("nicknames", `Nicks: ${this.data["nicks"]}`) +
                this.div("sex", `Gender: ${this.data["sex"]}`) +
                this.div("phone", `Phone: ${this.data["phone"]}`) +
                this.div("notes", `Notes: ${this.data["notes"]}`) +
                this.div("emergency", `Emergency: ${this.data["emergency"]}`)
            )
        );
        cb(result);
    }
}
