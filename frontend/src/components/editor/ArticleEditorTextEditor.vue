<template>
    <transition name="component-load" mode="out-in">
        <div id="content-wrapper">
            <div id="content">
                <article class="article text-editor-wrapper" contenteditable="true"
                         ref="article_content" @click.self.prevent @keydown.enter.prevent="enterClick()" @input="inputText">
                    <section class="headline">
                        <p ref="title" placeholder="Название статьи" id="title" class="title"></p>
                        <p ref="subtitle" placeholder="Подзаголовок статьи" id="subtitle" class="subtitle"></p>
                    </section>
                    <section id="article-section" class="article-content">
                        <p :id="'paragraph-'+index"
                           placeholder="Текст статьи" class="paragraph"
                           v-for="(content, index) in paragraphs"
                           :ref="'paragraph-'+index"
                           :key="`f${(~~(Math.random()*1e8)).toString(16)}`+index">{{content}}</p>
                    </section>
                </article>
            </div>
        </div>
    </transition>
</template>

<script>
    export default {
        name: "ArticleEditorTextEditor",
        data() {
            return { //Мб сделать через computed
                paragraphs: [''],
                selectedElement: undefined,
                selection: undefined,
                selectionRange: undefined
            }
        },
        mounted() {
            console.log(this.paragraphs)
            document.onselectionchange = () => {
                this.selection = window.getSelection
                this.selectionRange = this.selection.getRangeAt(0)
                if (this.selectedElement) {
                    this.selectedElement.classList.remove('selected')
                }
                this.selectedElement = this.selectionRange.commonAncestorContainer.id ?
                    this.selectionRange.commonAncestorContainer :
                    this.selectionRange.commonAncestorContainer.parentNode
                this.selectedElement.classList.add('selected')
            }
        },
        methods: {
            enterClick: function () {
                if (this.selectedElement.tagName === 'P') {
                    this.paragraphs.splice(this.selectedElement.id.replace('paragraph-', '')+1, 0, '')
                } else {
                    console.log(this.selectedElement.nextSibling)
                    this.selectedElement.nextElementSibling.focus()
                }
            },
            inputText: function () {
                if (this.selectedElement.tagName === 'P'){
                    this.paragraphs[this.selectedElement.id.replace('paragraph-', '')] = this.selectedElement.innerText
                }
            }
        },
    }
</script>

<style lang="sass" scoped>
    @import "../../variables"
    @import "../../article_style"
    [placeholder]:empty:after
        content: attr(placeholder)
        //display: block
        position: relative
        color: gray
        background: transparent
        -ms-user-select: none
        -moz-user-select: none
        -webkit-user-select: none
        user-select: none
        pointer-events: none
        height: 100%

    .text-editor-wrapper
        .article-content
            -webkit-user-select: contain
            -moz-user-select: contain
            -ms-user-select: contain
            user-select: contain

        .headline
            display: flex
            flex-direction: column

            span
                position: relative

        .paragraph
            &:first-child
                margin-top: 20px
</style>
