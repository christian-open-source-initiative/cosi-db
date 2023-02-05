/**
 * Manages the action bar in the dashboard that can spawn miniboard states.
 */

class ActionToolbar {
    constructor(miniboard, actionDom) {
        this.actionDom = actionDom;
        this.miniboard = miniboard;
        this.selected = [];
        this.displayButtons = false;

        $("#add-row").click(() => {
            if (!this.displayButtons) { return ;}
            this.miniboard.addState(ACTION_UPDATE, new PersonState());
        });
    }

    showButtons() {
        this.actionDom.show();
        this.displayButtons = true;
    }
}
