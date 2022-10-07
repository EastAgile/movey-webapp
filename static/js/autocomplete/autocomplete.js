const NO_MATCHES_FOUND = ["No matches found", "", ""];

class AutoComplete {
  // if isMain is false, type of auto complete will be inline
  constructor(
    id,
    getSuggestions,
    selectSuggestion,
    isMain = true,
    placeholder = "Enter a search term",
    defaultSearch = {},
    goBackCallback = () => {}
  ) {
    this.container = document.querySelector("#" + id);
    this.getSuggestions = getSuggestions;
    this.selectSuggestion = selectSuggestion;
    this.suggestions = [];
    this.placeholder = placeholder;
    this.currentChoiceIndex = -1;
    this.inputValue = "";

    if (isMain) this.displayMain();
  }

  reDisplay(hiddenSuggestions = false) {
    const input = this.container.querySelector("input");
    const xButton = this.container.querySelector("#button-x");
    const wrapper = this.container.querySelector("div");
    const suggestionsContainer = this.container.querySelector("#suggestions");

    if (xButton) {
      if(!hiddenSuggestions)
        if (input.value) {
          xButton.classList.remove("hidden");
        }
      else
        xButton.classList.add("hidden");
    }

    if (
      hiddenSuggestions ||
      !Array.isArray(this.suggestions) ||
      this.suggestions.length <= 0
    ) {
      input.classList.remove("input-has-item");
      wrapper.classList.remove("autocomplete-shadow");
      suggestionsContainer.classList.add("hidden");
      return;
    }

    suggestionsContainer.innerHTML = "";
    suggestionsContainer.classList.remove("hidden");

    input.classList.add("input-has-item");
    wrapper.classList.add("autocomplete-shadow");

    // let descriptionWidth = $(".autocomplete-main-wrapper").width() - 120;

    this.suggestions.forEach((suggestion, index) => {
      const option = this.getOptionValue(suggestion[0]);

      const packageName = document.createElement("div");
      packageName.setAttribute("class", "package-name");
      packageName.innerHTML = option;

      const packageDescription = document.createElement("div");
      packageDescription.setAttribute("class", "package-description");
      // packageDescription.setAttribute("style", "width:" + descriptionWidth + "px");
      packageDescription.innerHTML = suggestion[1];

      const packageVersion = document.createElement("div");
      packageVersion.setAttribute("class", "package-version");
      packageVersion.innerHTML = suggestion[2];

      const packageStarsAndForks = document.createElement("div");
      packageStarsAndForks.setAttribute("class", "package-stars-and-forks");

      // stars count
      const starsCountContainer = document.createElement("div");
      starsCountContainer.setAttribute("class", "stars-count");
      
      const starIcon = document.createElement("img");
      starIcon.setAttribute("src", "/static/resources/star-active.svg");

      const starsCount = document.createElement("b");
      starsCount.innerHTML = suggestion[4];

      starsCountContainer.appendChild(starIcon);
      starsCountContainer.appendChild(starsCount);

      packageStarsAndForks.appendChild(starsCountContainer);

      // forks count
      const forksCountContainer = document.createElement("div");
      forksCountContainer.setAttribute("class", "forks-count");

      const forkIcon = document.createElement("img");
      forkIcon.setAttribute("src", "/static/resources/fork-active.svg");

      const forksCount = document.createElement("b");
      forksCount.innerHTML = suggestion[5];

      forksCountContainer.appendChild(forkIcon);
      forksCountContainer.appendChild(forksCount);

      packageStarsAndForks.appendChild(forksCountContainer);

      const node = document.createElement("div");
      const suggesstionContent = document.createElement("div");
      suggesstionContent.setAttribute("class", "suggestion-content");
      node.setAttribute('id', 'suggestion' + index)
      node.setAttribute('style', 'display: flex');
      suggesstionContent.appendChild(packageName);
      suggesstionContent.appendChild(packageDescription);

      const suggestionExtraContent = document.createElement("div");
      suggestionExtraContent.setAttribute("class", "suggestion-extra-content");
      suggestionExtraContent.appendChild(packageVersion);
      if (option !== NO_MATCHES_FOUND[0]) {
        console.log(option);
        suggestionExtraContent.appendChild(packageStarsAndForks);
      }

      node.appendChild(suggesstionContent);
      node.appendChild(suggestionExtraContent);

      if (this.currentChoiceIndex === index)
        node.classList.add("autocomplete-item-hover");

      // add event click, mouseover and mouse leave
      if (option !== NO_MATCHES_FOUND[0]) {
        node.addEventListener("click", () => {
          input.value = option;
          this.reDisplay(true);
          // use package slug instead of package name for package detail url
          this.selectSuggestion(suggestion[3]);
        });

        node.addEventListener("mouseover", () => {
          this.getSuggestionNode(this.currentChoiceIndex)?.classList.remove(
            "autocomplete-item-hover"
          );
          node.classList.add("autocomplete-item-hover");
          this.currentChoiceIndex = index;
        });
        node.addEventListener("mouseout", () => {
          node.classList.remove("autocomplete-item-hover");
        });
      } else {
        node.classList.add("no-pointer");
      }
      suggestionsContainer.appendChild(node);
    });

  }

  getOptionValue(suggestion) {
    return typeof suggestion === "object" ? Object.values(suggestions) : suggestion;
    // Recheck in future
    // return typeof suggestion === "object" ? suggestion?.option : suggestion;
  }

  getSuggestionNode(id) {
    return this.container.querySelector("#suggestions #suggestion" + id);
  }

  checkKeyDown(event) {
    const key = event.key;

    const keyDownSpecial = ["ArrowUp", "ArrowDown", "Enter"];
    const emptySuggestion = this.suggestions[0] === NO_MATCHES_FOUND;
    const input = this.container.querySelector("input");
    if (key === "Enter" && !emptySuggestion) {
      // Not having any suggestion, goes to search page
      window.location.href = '/packages/search?query='+input.value;
    }
    if (!keyDownSpecial.includes(key)) return;

    const noSuggestions = this.suggestions.length;
    const pChoiceIndex = this.currentChoiceIndex;
    const pChoice = this.getSuggestionNode(pChoiceIndex);

    if (key === "ArrowUp") {
      this.currentChoiceIndex =
        pChoiceIndex >= 0 ? pChoiceIndex - 1 : noSuggestions - 1;
    } else if (key === "ArrowDown") {
      this.currentChoiceIndex =
        pChoiceIndex + 1 < noSuggestions ? pChoiceIndex + 1 : -1;
    } else if (key === "Enter") {
      if (pChoiceIndex === -1) {
        // Not choosing any suggestion, goes to search page
        window.location.href = '/packages/search?query='+input.value;
      } else {
        // Goes to highlighted package
        pChoice.click();
      }
      return;
    }

    event.preventDefault();

    if (this.currentChoiceIndex === -1) {
      input.value = this.inputValue;
    } else {
      input.value = this.suggestions[this.currentChoiceIndex][0];
    }

    const currentChoice = this.getSuggestionNode(this.currentChoiceIndex);
    pChoice?.classList.remove("autocomplete-item-hover");
    currentChoice?.classList.add("autocomplete-item-hover");
    currentChoice?.scrollIntoView({behavior: "smooth", block: "end", inline: "nearest"})
  }

  bindListeners() {
    const input = this.container.querySelector("input");
    input.addEventListener("input", async (e) => {
      if (!input.value) {
        this.suggestions = [];
      } else {
        try {
          let suggestions = await this.getSuggestions(e.target.value);
          suggestions.length > 0 ? this.suggestions = suggestions :
            input.value.length >= 3 ? this.suggestions = [NO_MATCHES_FOUND] : this.suggestions = [];
        } catch (error) {
          this.suggestions = [NO_MATCHES_FOUND];
        }
      }
      this.currentChoiceIndex = -1;
      this.inputValue = input.value;
      this.reDisplay();
    });

    input.addEventListener("keydown", (e) => this.checkKeyDown(e));

    input.addEventListener("focusin", () => this.reDisplay(false));
    input.addEventListener("focusout", () => {
      if(!$(this.container.querySelector("#suggestions")).is(":hover")) {
        window.setTimeout(() => this.reDisplay(true), 50)
      }
    });
  }

  displayMain() {
    this.container.innerHTML = `
    <div class="autocomplete-main-wrapper">
      <div class="autocomplete-main">
        <input
          class="gray"
          type="text"
          id="search-bar"
          placeholder="${this.placeholder}"
        />

        <button class="icon-default-main" id="button-main"><i class="fa fa-search"></i></button>
        <button class="icon-right-main hidden" id="button-x"><img class="icon" src="/static/resources/x-button.svg"/></i></button>
        <div id="suggestions" class="autocomplete-items autocomplete-shadow hidden"></div>
      </div>
    </div>`;

    this.bindListeners();

    const input = this.container.querySelector("input");
    const button = this.container.querySelector("#button-main");
    button.addEventListener("click", () => {
      if(input.value) {
        window.location.href = '/packages/search?query='+input.value;
      }
    });
    const xButton = this.container.querySelector("#button-x");
    xButton.addEventListener("click", () => {
      input.value = "";
      this.suggestions = [];
      xButton.classList.add("hidden");
    });
  }
}
