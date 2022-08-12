class SearchManager {
    constructor(searchBar, searchSuggestion, searchButton, searchDarkener) {
        this.searchBar = searchBar;
        this.searchSuggestion = searchSuggestion;
        this.searchButton = searchButton;
        this.searchDarkener = searchDarkener;

        // Initial states
        this.searchSuggestion.hide();
        this.searchDarkener.hide();
        // We don't want to spam the server
        this.currentQuery = "";

        // Requires binding due to function reference.
        this.searchBar.keyup(
            (event) => {
                if (event.key == "Escape") {
                    this.searchDarkener.hide();
                    this.searchSuggestion.hide();
                    // Unfocus so that we can refocus if start typing.
                    this.searchDarkener.blur();
                    return;
                }
                this.determineHide();
                if (this.currentQuery.length != this.searchBar.val().length) {
                    this.currentQuery = this.searchBar.val();
                    this.dispatchSearch();
                }
            }
        );

        // We only want to hid if user focuses and already typed.
        this.searchBar.focus(this.determineHide.bind(this));
        this.searchBar.blur(() => { this.searchDarkener.hide(); this.searchSuggestion.hide(); });
    }

    dispatchSearch() {
        if (this.currentQuery.length < 3) {
            return;
        }

        // Dispatches the search result to all available tables.
        console.log(this.currentQuery)
        $.get(`/search?query=${this.currentQuery}`, (data) => {
            this.searchSuggestion.empty();
            console.log(data);
            for (let table_key in data) {
                let results = data[table_key];
                for (let r of results) {
                    this.searchSuggestion.append(`${table_key}: ${JSON.stringify(r)}<br /><br />`);
                    this.searchSuggestion.show();
                }
            }
        }).fail((d, textStatus, error) => {console.log(error);});
    }

    determineHide() {
        if (this.searchBar.val() == "") {
            this.searchDarkener.hide();
            this.searchSuggestion.hide();
            return true;
        }

        this.searchSuggestion.show();
        this.searchDarkener.show();
    }
}
