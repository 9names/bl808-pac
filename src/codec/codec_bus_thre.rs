#[doc = "Register `codec_bus_thre` reader"]
pub struct R(crate::R<CODEC_BUS_THRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CODEC_BUS_THRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CODEC_BUS_THRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CODEC_BUS_THRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `codec_bus_thre` writer"]
pub struct W(crate::W<CODEC_BUS_THRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CODEC_BUS_THRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CODEC_BUS_THRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CODEC_BUS_THRE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_x_wthre_blai2sysram` reader - "]
pub type REG_X_WTHRE_BLAI2SYSRAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_x_wthre_blai2sysram` writer - "]
pub type REG_X_WTHRE_BLAI2SYSRAM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CODEC_BUS_THRE_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_x_wthre_blai2ext` reader - "]
pub type REG_X_WTHRE_BLAI2EXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_x_wthre_blai2ext` writer - "]
pub type REG_X_WTHRE_BLAI2EXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CODEC_BUS_THRE_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_x_wthre_vdo2pb` reader - "]
pub type REG_X_WTHRE_VDO2PB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_x_wthre_vdo2pb` writer - "]
pub type REG_X_WTHRE_VDO2PB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CODEC_BUS_THRE_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_x_wthre_vdo2pa` reader - "]
pub type REG_X_WTHRE_VDO2PA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_x_wthre_vdo2pa` writer - "]
pub type REG_X_WTHRE_VDO2PA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CODEC_BUS_THRE_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_x_wthre_vdo2sysram` reader - "]
pub type REG_X_WTHRE_VDO2SYSRAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_x_wthre_vdo2sysram` writer - "]
pub type REG_X_WTHRE_VDO2SYSRAM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CODEC_BUS_THRE_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_10_31` reader - "]
pub type RESERVED_10_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn reg_x_wthre_blai2sysram(&self) -> REG_X_WTHRE_BLAI2SYSRAM_R {
        REG_X_WTHRE_BLAI2SYSRAM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_x_wthre_blai2ext(&self) -> REG_X_WTHRE_BLAI2EXT_R {
        REG_X_WTHRE_BLAI2EXT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn reg_x_wthre_vdo2pb(&self) -> REG_X_WTHRE_VDO2PB_R {
        REG_X_WTHRE_VDO2PB_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn reg_x_wthre_vdo2pa(&self) -> REG_X_WTHRE_VDO2PA_R {
        REG_X_WTHRE_VDO2PA_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn reg_x_wthre_vdo2sysram(&self) -> REG_X_WTHRE_VDO2SYSRAM_R {
        REG_X_WTHRE_VDO2SYSRAM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:31"]
    #[inline(always)]
    pub fn reserved_10_31(&self) -> RESERVED_10_31_R {
        RESERVED_10_31_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_x_wthre_blai2sysram(&mut self) -> REG_X_WTHRE_BLAI2SYSRAM_W<0> {
        REG_X_WTHRE_BLAI2SYSRAM_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_x_wthre_blai2ext(&mut self) -> REG_X_WTHRE_BLAI2EXT_W<2> {
        REG_X_WTHRE_BLAI2EXT_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_x_wthre_vdo2pb(&mut self) -> REG_X_WTHRE_VDO2PB_W<4> {
        REG_X_WTHRE_VDO2PB_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_x_wthre_vdo2pa(&mut self) -> REG_X_WTHRE_VDO2PA_W<6> {
        REG_X_WTHRE_VDO2PA_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn reg_x_wthre_vdo2sysram(&mut self) -> REG_X_WTHRE_VDO2SYSRAM_W<8> {
        REG_X_WTHRE_VDO2SYSRAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "codec_bus_thre\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [codec_bus_thre](index.html) module"]
pub struct CODEC_BUS_THRE_SPEC;
impl crate::RegisterSpec for CODEC_BUS_THRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [codec_bus_thre::R](R) reader structure"]
impl crate::Readable for CODEC_BUS_THRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [codec_bus_thre::W](W) writer structure"]
impl crate::Writable for CODEC_BUS_THRE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets codec_bus_thre to value 0"]
impl crate::Resettable for CODEC_BUS_THRE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
