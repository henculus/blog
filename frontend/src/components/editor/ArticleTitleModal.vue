<template>
    <div class="modal__wrapper">
        <div class="modal__form">
            <label class="modal__form__label" for="modal__form__input">Название статьи</label>
            <section class="input__wrapper">
                <input class="modal__form__input" id="modal__form__input" type="text" v-model="title">
                <transition name="mark" appear v-if="title">
                    <button class="mark" @click="$emit('setTitle', title)">
                        <mark-icon class="mark__icon"/>
                    </button>
                </transition>
            </section>
        </div>
    </div>
</template>

<script>
    import MarkIcon from "../icons/MarkIcon"

    export default {
        name: "ArticleTitleModal",
        components: {
            MarkIcon
        },
        data() {
            return {
                title: '',
            }
        }
    }
</script>

<style lang="sass" scoped>
    @import "../../variables"


    .modal
        &__wrapper
            z-index: 99
            width: 100%
            height: 100%
            position: absolute
            left: 0
            top: 0
            background-color: rgba(0, 0, 0, 0.7)
            display: flex
            flex-direction: row
            justify-content: center

        &__form
            position: relative
            margin-top: 150px
            display: flex
            flex-direction: column
            align-items: center
            width: 450px
            padding: 50px
            color: $rnt_green
            font-family: 'Comfortaa', sans-serif

            &__label
                font-size: 1.6em
                font-weight: 400
                margin: 16px 0

            &__input
                font-size: 1.2em
                border: none
                color: white
                background: rgba(255, 255, 255, 0.2)
                padding: 15px 50px 15px 20px
                border-radius: 100px
                transition: all .3s ease
                width: 100%

    .input__wrapper
        display: flex
        flex-direction: row
        position: relative
        width: 100%

    .mark
        border: none
        position: absolute
        right: 6px
        top: 6px
        background: transparent
        width: 40px
        height: 40px
        border-radius: 100%
        padding: 8px
        cursor: pointer
        transition: all .2s ease
        &__icon
            fill: $rnt_green
        &:hover
            background: $rnt_green_light


    .mark-enter, .mark-leave-to
        opacity: 0

    .mark-enter-to, .mark-leave
        opacity: 1

    .mark-enter-active, .mark-leave-active
        transition: all .2s ease
</style>
