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
        this.StateConstructor = null;

        $("#add-row").click(() => {
            if (!this.displayButtons) { return ;}
            else if (this.StateConstructor == null ) { alert("Operator not yet supported for this table."); return; }
            this.miniboard.addState(ACTION_INSERT, new this.StateConstructor());
        });

        $("#update-row").click(() => {
            if (!this.displayButtons || this.selected.length <= 0) { return ;}
            else if (this.StateConstructor == null ) { alert("Operator not yet supported for this table."); return; }

            let params = {};
            // Assume single selected for now.
            let oid = this.selected[0];
            $(`[oid=${oid}] td`).each(function() {
                let dom = $(this);
                params[dom.attr("entry-name")] = dom.text();
            });
            // Used for updating the result.
            params._oid = oid;
            this.miniboard.addState(ACTION_UPDATE, new this.StateConstructor(params));
        });

        $("#delete-row").click(()=> {
            if (!this.displayButtons || this.selected.length <= 0) { return ;}
            else if (this.StateConstructor == null ) { alert("Operator not yet supported for this table."); return; }

            let params = {};
            // Assume single selected for now.
            let oid = this.selected[0];
            $(`[oid=${oid}] td`).each(function() {
                let dom = $(this);
                params[dom.attr("entry-name")] = dom.text();
            });
            // Used for updating the result.
            params._oid = oid;
            this.miniboard.addState(ACTION_CAT, new this.StateConstructor(params));
        })
    }

    setState(state) {
        this.StateConstructor = state;
    }

    showButtons() {
        this.actionDom.show();
        this.displayButtons = true;
    }

    update() {
        // Updates the display of the action bar.
        if (this.selected.length == 0) {
            $("#action-toolbar-more").fadeOut(100);
            $("#action-toolbar-message").html("");
        } else {
            $("#action-toolbar-more").fadeIn(120).css("display", "inline");
            $("#action-toolbar-message").html("selected 1 row(s):");
        }
    }

    setSelected(oid) {
        if (!oid) {
            this.selected = [];
        }
        else {
            // For now, only support a single select
            this.selected = [oid];
        }
        this.update();
    }
}
