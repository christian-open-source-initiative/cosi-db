/**
 * Manages the action bar in the dashboard that can spawn miniboard states.
 */

class ActionToolbar {
    constructor(miniboard, actionDom) {
        this.actionDom = actionDom;
        this.miniboard = miniboard;
        this.selected = [];
        this.displayButtons = false;
        // Should be an instance of FormStruct
        this.state = null;

        $("#add-row").click(() => {
            if (!this.displayButtons) { return ;}
            this.miniboard.addState(ACTION_INSERT, this.state);
        });
    }

    setState(state) {
        this.state = state;
    }

    showButtons() {
        this.actionDom.show();
        this.displayButtons = true;
    }
}
