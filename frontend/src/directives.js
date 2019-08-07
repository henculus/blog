import Vue from 'vue'

Vue.directive('click-outside', {
    bind(el, binding, vnode) {
        let excluded_element = binding.value.exclude;
        el.clickOutsideEvent = function (event) {
            if (!(el === event.target || el.contains(event.target) || vnode.context.$refs[excluded_element] === event.target)) {
                vnode.context[binding.value.isShown] = false
            }
        };
        document.body.addEventListener('mousedown', el.clickOutsideEvent);
    },
    unbind(el) {
        document.body.removeEventListener('mousedown', el.clickOutsideEvent);
    }
});
