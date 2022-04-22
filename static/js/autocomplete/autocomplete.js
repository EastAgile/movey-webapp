const NO_MATCHES_FOUND = "No matches found";

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
    this.isMain = isMain;
    this.suggestions = [];
    this.placeholder = placeholder;
    this.currentChoiceIndex = -1;
    this.inputValue = "";
    this.defaultSearch = defaultSearch;
    this.goBackCallback = goBackCallback;

    if (isMain) this.displayMain();
    else this.displayInline();
  }

  reDisplay(hiddenSuggestions = false) {
    const input = this.container.querySelector("input");
    const button = this.container.querySelector("button");
    const wrapper = this.container.querySelector("div");
    const suggestionsContainer = this.container.querySelector("#suggestions");

    if (button) {
      if (input.value) {
        if(hiddenSuggestions)
          button.classList.remove("hidden");
        else
          button.classList.add("hidden");
      }
      else button.classList.remove("hidden");
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
    this.suggestions.forEach((suggestion, index) => {
      const option = this.getOptionValue(suggestion);

      const node = document.createElement("div");
      node.setAttribute("id", "suggestion" + index);
      node.innerHTML = option;

      if (this.currentChoiceIndex === index)
        node.classList.add("autocomplete-item-hover");

      // add event click, mouseover and mouse leave
      if (option !== NO_MATCHES_FOUND) {
        node.addEventListener("click", () => {
          input.value = option;
          this.reDisplay(true);
          this.selectSuggestion(suggestion);
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
    return typeof suggestion === "object" ? suggestion?.option : suggestion;
  }

  getSuggestionNode(id) {
    return this.container.querySelector("#suggestions #suggestion" + id);
  }

  checkKeyDown(event) {
    const key = event.key;

    const keyDownSpecial = ["ArrowUp", "ArrowDown", "Enter"];
    const emptySuggestion = this.suggestions[0] === NO_MATCHES_FOUND;
    if (!keyDownSpecial.includes(key) || emptySuggestion) return;

    const input = this.container.querySelector("input");
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
      pChoiceIndex !== -1 && pChoice.click();
      return;
    }

    event.preventDefault();
    if (this.currentChoiceIndex === -1) {
      input.value = this.inputValue;
    } else {
      input.value = this.getOptionValue(
        this.suggestions[this.currentChoiceIndex]
      );
    }

    const currentChoice = this.getSuggestionNode(this.currentChoiceIndex);
    pChoice?.classList.remove("autocomplete-item-hover");
    currentChoice?.classList.add("autocomplete-item-hover");
  }

  bindListeners() {
    const input = this.container.querySelector("input");
    input.addEventListener("input", async (e) => {
      if (!input.value) {
        this.suggestions = [];
      } else {
        try {
          let suggestions = await this.getSuggestions(e.target.value);
          suggestions.length > 0 ? this.suggestions = this.suggestions = suggestions : this.suggestions = [NO_MATCHES_FOUND];
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
    input.addEventListener("focusout", () =>
      window.setTimeout(() => this.reDisplay(true), 50)
    );
  }

  displayMain() {
    this.container.innerHTML = `
    <div class="autocomplete-main-wrapper">
      <div class="autocomplete-main">
        <input
          class="gray"
          type="text"
          placeholder="${this.placeholder}"
        />
        
        <button class="icon-default-main"><i class="fa fa-search"></i></button> 
        <div id="suggestions" class="autocomplete-items autocomplete-shadow hidden"></div>
      </div>
    </div>`;

    this.bindListeners();

    const input = this.container.querySelector("input");
    const button = this.container.querySelector("button");
    button.addEventListener("click", () => {
      input.value = "";
      this.suggestions = [];
      this.reDisplay();
    });
  }
}
