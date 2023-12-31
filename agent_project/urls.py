"""agent_project URL Configuration

The `urlpatterns` list routes URLs to views. For more information please see:
    https://docs.djangoproject.com/en/4.1/topics/http/urls/
Examples:
Function views
    1. Add an import:  from my_app import views
    2. Add a URL to urlpatterns:  path('', views.home, name='home')
Class-based views
    1. Add an import:  from other_app.views import Home
    2. Add a URL to urlpatterns:  path('', Home.as_view(), name='home')
Including another URLconf
    1. Import the include() function: from django.urls import include, path
    2. Add a URL to urlpatterns:  path('blog/', include('blog.urls'))
"""
from django.contrib import admin
from django.urls import path
from ninja import NinjaAPI, Schema

from agent_app.routers.gpt_stream import router as stream_router
from agent_app.routers.prompt_c import router as prompt_c_router
from agent_app.routers.guard_rails import router as guard_rail_router
from agent_app.routers.speech2text import router as speech2text_router

api = NinjaAPI()

api.add_router("/", stream_router)
api.add_router("/",prompt_c_router)
api.add_router("/",guard_rail_router)
api.add_router("/openai",speech2text_router)

urlpatterns = [
    path("admin/",
    admin.site.urls),
    path("api/", api.urls)
]
