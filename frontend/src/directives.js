import Vue from 'vue'

Vue.directive('click-outside', {
    bind(el, binding, vnode) {
        let excluded_element = binding.value.exclude
        el.clickOutsideEvent = function (event) {
            if (!(el === event.target || el.contains(event.target) || vnode.context.$refs[excluded_element] === event.target)) {
                vnode.context[binding.value.isShown] = false
            }
        }
        document.body.addEventListener('mousedown', el.clickOutsideEvent)
    },
    unbind(el) {
        document.body.removeEventListener('mousedown', el.clickOutsideEvent)
    }
})
Vue.directive('focus', {
    update: function (el) {
        Vue.nextTick(function () {
            console.log('v-focus worked')
            el.lastChild.focus()
            if (typeof window.getSelection != "undefined"
                && typeof document.createRange != "undefined") {
                let range = document.createRange()
                range.selectNodeContents(el.lastChild)
                range.collapse(false)
                let sel = window.getSelection()
                sel.removeAllRanges()
                sel.addRange(range)
            } else if (typeof document.body.createTextRange != "undefined") {
                let textRange = document.body.createTextRange()
                textRange.moveToElementText(el.lastChild)
                textRange.collapse(false)
                textRange.select()
            }
        })
    }
})
Vue.directive('getContent', {
    bind: function (el, binding, vnode) {
        el.addEventListener('input', function () {
            Vue.nextTick(function () {
                for (let i = el.children.length - 1; i >= 0; i--) {
                    vnode.context.paragraphs.length = el.children.length
                    vnode.context.paragraphs[i] = el.children[i].innerText
                }
            })


        })
    },
})
