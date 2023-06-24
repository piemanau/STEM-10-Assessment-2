// Lets me update a value to a HTML element from rust, limitation of yew that doesnt let the change render
export function updateValue(element, value) {
    element.value = value;
}