<template>
    <div class="observer">
        <slot :displayed = "displayed">

        </slot>
    </div>
</template>

<script>
    export default {
        name: "IntersectionObserver",
        data(){
            return{
                displayed: false
            }
        },
        mounted() {
            this.$nextTick(function () {
                let self = this
                let callback = function (entries, observer) {
                    if(entries[0].isIntersecting) {
                        self.displayed = true
                        observer.disconnect()
                    }
                }
                let observer = new IntersectionObserver(callback, {rootMargin: '0px 0px 50px 0px', threshold: 0})
                observer.observe(this.$el.firstChild)
            })
        }
    }
</script>

<style scoped>

</style>
