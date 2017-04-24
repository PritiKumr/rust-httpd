$ ->
  $('.entry-editor').each (i, el) ->
    setupEditor el

  flatpickr ".datepicker", 
    wrap: true
    maxDate: new Date()
    onChange: (_, date) ->
      Turbolinks.visit date

  $('.entry-authors .author-name').on 'click', (e) ->
    tab = $(@)
    entryId = tab.data('entry')
    console.log entryId
    $('.entry-authors, .entry').removeClass 'active'
    tab.parent().addClass 'active'
    $(entryId).addClass 'active'

setupEditor = (container) ->
  lockEditing = container.getAttribute('data-disabled') == 'true'

  editorOptions = 
    disableEditing: lockEditing
    toolbar: not lockEditing
    placeholder: false

  editor = new MediumEditor container, editorOptions

  editor.subscribe 'editableInput', DuoUtils.debounce (event, editorElement) ->
    $.ajax 
      url: container.getAttribute('data-url')
      data:
        entry:
          text: editorElement.innerHTML
      method: 'PUT'
      dataType: 'json'
  , 2000